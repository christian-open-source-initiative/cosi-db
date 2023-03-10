# Importer to import data from external database.
import os
import json
import requests
import pandas
from collections import defaultdict

API_URL = "http://127.0.0.1:8000"
TABLE_NAMES = [
    "Person",
    "Address",
    "Household",
    "Group",
    "GroupRelation",
    "Event",
    "EventRegistration"
]

def cosi_get(endpoint, session, params=None):
    response = session.get(os.path.join(API_URL, endpoint), params=params)
    return response.json()

def cosi_post(endpoint, session, params=None):
    response = session.post(os.path.join(API_URL, endpoint), data=params)
    parsed = response.json()
    assert "err" not in parsed, parsed["err"]
    return parsed

def login(user, password, session):
    cosi_post("login", session, params={"email": user, "token": password})

def get_gender(g_str):
    if g_str == "M":
        return "Male"
    elif g_str == "F":
        return "Female"
    return "Undefined"

def nan_to_empty(v):
    if str(v) == "nan":
        return ""
    return v

def nan_to_none(v):
    if str(v) == "nan":
        return None
    return v

def import_person(people_df, session):
    # Import people
    person_track = {}
    for _, person in people_df.iterrows():
        f_date = person["Birth Date"]
        if str(f_date) != "nan":
            date = person["Birth Date"].split("/")
            f_date = "-".join([date[2], date[0], date[1]])
        else:
            f_date = None

        p = {
            "first_name": person["FirstName"],
            "middle_name": "",
            "last_name": person["LastName"],
            "dob": f_date,
            "home_phone": nan_to_none(person["HomePhone"]),
            "mobile_phone": nan_to_none(person["MobilePhone"]),
            "work_phone": nan_to_none(person["WorkPhone"]),
            "sex": get_gender(person["Gender"]),
            "notes": nan_to_empty(person["Allergy/child notes"]),
            "emergency_contact": nan_to_empty(person["Emergency Contact"])
        }

        result = cosi_post("insert_person", session=session, params=p)
        key = (p["first_name"], p["last_name"], person["Address1"])
        assert key not in person_track, f"Duplicate name: {key}"
        assert "$oid" in result
        person_track[key] = result["$oid"]
    return person_track


def import_address(people_df, person_oid_track, session):
    household_track = defaultdict(list)
    for _, person in people_df.iterrows():
        a = {
            "line_one": nan_to_empty(person["Address1"]),
            "line_two": nan_to_empty(person["Address2"]),
            "line_three": "",
            "city": nan_to_empty(person["City"]),
            "region": nan_to_empty(person["State"]),
            "postal_code": nan_to_empty(person["ZipCode"]),
            "country": "United States",
        }

        if a["line_one"] == "":
            continue

        # Check if db already exists this value
        res = cosi_get("find_address", session=session, params=a)
        address_oid = None
        if len(res["data"]) == 0:
            res = cosi_post("insert_address", session=session, params=a)
            address_oid = res["$oid"]
        else:
            assert len(res["data"]) == 1
            address_oid = res["data"][0]["_id"]["$oid"]

        # Don't generate households until the very end.
        # Grab household if it doesn't exist
        household_track[address_oid].append(person_oid_track[(person["FirstName"], person["LastName"], person["Address1"])])

    p_keys = list(person_oid_track.keys())
    p_values = list(person_oid_track.values())
    for address_oid, people in household_track.items():
        assert len(set(people)) == len(people), "Duplicate names? {}".format(people)
        # Set household name to the last name of the entire family.
        last_names = []
        for poid in people:
            last_names.append(p_keys[p_values.index(poid)][1])

        household_name = max(set(last_names), key=last_names.count)
        res = cosi_post("insert_household", session=session, params={
            "house_name": household_name + " Household",
            "address": address_oid,
            "persons": people
        })

        assert "err" not in res, res


def import_group(group_df, person_oid_track, session):
    group_desc = {}
    group_track = defaultdict(list)
    for _, row in group_df.iterrows():
        group_track[row["GroupName"]].append([(row["FirstName"], row["LastName"], row["Address1"]), row["GroupMemberType"]])
        group_desc[row["GroupName"]] = row["GroupDescription"]

    for group_name, tups in group_track.items():
        g = {
            "group_name": group_name,
            "group_desc": group_desc[group_name]
        }

        res = cosi_post("insert_group", session, g)
        oid = res["$oid"]
        for people in tups:
            pg = {
                "person": person_oid_track[people[0]],
                "group": oid,
                "role": people[1]
            }

            cosi_post("insert_grouprelation", session, pg)

def main():
    with requests.sessions.Session() as session:
        login("admin@projectcosi.org", "admin", session)

        # Drop data
        print("**Dropping Tables**")
        for k in [t.lower() for t in TABLE_NAMES]:
            cosi_get(f"drop_{k}", session=session)
            # Double check all data is wiped
            result = cosi_get(f"find_{k}", session=session, params={"page": 0})
            assert len(result["data"]) == 0
            print(f"Dropped: {k}")
        print()

    print("Reading File")
    people_file = "people.csv"
    groups_file = "groups.csv"
    people_df = pandas.read_csv(people_file, encoding="ISO-8859-1")
    group_df = pandas.read_csv(groups_file, encoding="ISO-8859-1")

    print("Importing Person Data")
    person_oid_track = import_person(people_df, session=session)

    print("Importing Address Data")
    import_address(people_df, person_oid_track, session=session)

    print("Importing Group Data")
    import_group(group_df, person_oid_track, session=session)


if __name__ == "__main__":
    main()
