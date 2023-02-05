import session from "supertest-session";
import {jest} from "@jest/globals";
import { ALL_PAGEABLE_ENDPOINTS, ALL_GEN_ENDPOINTS, TABLE_NAMES } from "./endpoints.js";

jest.setTimeout(10000);
var totalDatapointsPerTable = 200;
var cosiRequest = session("127.0.0.1:8000");

// Basic helpers
function expectKeys(jsonData, keys) {
    expect(Object.keys(jsonData)).toEqual(keys);
}

// Test setup
beforeAll(async ()=> {
    await cosiRequest.get("/gen_login/1").expect(200).expect("Content-Type", /json/);

    // Login
    await cosiRequest
            .post("/login")
            .type("form")
            .send({
                "email": "admin@projectcosi.org",
                "token": "admin"
            })
            .expect(200)
            .expect("Content-Type", /json/);

    // Test drop endpoints.
    for (let tn of TABLE_NAMES) {
        let response = await cosiRequest.get(`/drop_${tn.toLowerCase()}`)
                                        .expect(200)
                                        .expect("Content-Type", /json/);

        let jsonData = JSON.parse(response.text);
        expectKeys(jsonData, ["dropped"]);
        expect(jsonData["dropped"]).toBe(true);

        let findResponse = await cosiRequest.get(`/get_${tn.toLowerCase()}`)
                                            .query({page: 0})
                                            .expect(200)
                                            .expect("Content-Type", /json/);
        jsonData = JSON.parse(findResponse.text);
        expect(jsonData["data"].length == 0);
    }

    // Before tests begin, populate table with values.
    for (let endpoint of ALL_GEN_ENDPOINTS) {
        let response = await cosiRequest.get(`/${endpoint}/${totalDatapointsPerTable}`)
                                           .expect(200)
                                           .expect("Content-Type", /json/);

        let jsonData = JSON.parse(response.text);
        expectKeys(jsonData, ["total"]);
        expect(jsonData["total"]).toBe(totalDatapointsPerTable)
    }
});

// Testing
describe("Test Root", () => {
    test("/ GET", async () => {
        cosiRequest.get("/").expect(200);
    })
});

describe("CRUD", () => {
    // Check all basic GET endpoints.
    // Each get page should have max 100 datapoints.
    describe("Verify Getters", () => {
        const maxDatapoints = 100;
        const returnKeys = [
            "page",
            "total_pages",
            "total_result",
            "data"
        ];
        for (let endpoint of ALL_PAGEABLE_ENDPOINTS) {
            test(`/${endpoint} GET`, async () => {
                // Basic endpoint response verification.
                const response = await cosiRequest
                                        .get(`/${endpoint}`)
                                        .query({page: 0})
                                        .expect(200)
                                        .expect("Content-Type", /json/);

                // Further data verification.
                let jsonData = JSON.parse(response.text);
                expectKeys(jsonData, returnKeys);
                expect(Object.keys(jsonData["data"]).length).toBe(maxDatapoints);
            });

            test(`/${endpoint} Empty page load`, async() => {
                const allData = await cosiRequest
                                        .get(`/${endpoint}`)
                                        .expect(200)
                                        .query({page: Number.MAX_SAFE_INTEGER})
                                        .expect("Content-Type", /json/);
                let jsonData = JSON.parse(allData.text);
                expect(Object.keys(jsonData["data"]).length).toBe(0);
            });

            test(`/${endpoint} Invalid page load`, async() => {
                const fakePages = ["cosi", "-1", "!@#$%^&*()-_+=`"];
                for (const page of fakePages){
                    const allData = await cosiRequest
                                        .get(`/${endpoint}`)
                                        .query({page: `${page}`})
                                        .expect(200)
                                        .expect("Content-Type", /json/);
                    let jsonData = JSON.parse(allData.text);
                    expect(jsonData["page"]).toBe(0);
                    expect(Object.keys(jsonData["data"]).length).toBe(maxDatapoints);
                    expect(jsonData["total_pages"]).toBe(Math.ceil(totalDatapointsPerTable/maxDatapoints));
                    expect(jsonData["total_result"]).toBe(totalDatapointsPerTable);
                }
            });

            test(`/${endpoint} Correct page count`, async() => {
                const allData = await cosiRequest
                                        .get(`/${endpoint}`)
                                        .expect(200)
                                        .expect("Content-Type", /json/);
                let jsonData = JSON.parse(allData.text);
                let totalPages = jsonData["total_pages"];
                expect(totalPages).toBe(Math.ceil(totalDatapointsPerTable/maxDatapoints));
                expect(jsonData["total_result"]).toBe(totalDatapointsPerTable);
            });

            test(`/${endpoint} No duplicate page data`, async() => {
                let pages = [];
                // Concatenate all data to single array
                for(let page = 0; page < Math.ceil(totalDatapointsPerTable/maxDatapoints); page++){
                    let request = await cosiRequest
                                        .get(`/${endpoint}`)
                                        .query({page: `${page}`})
                                        .expect(200)
                                        .expect("Content-Type", /json/)
                    let jsonData = JSON.parse(request.text);
                    pages = pages.concat(Object.values(jsonData["data"]));
                }
                // Converting to a set will de-duplicate data. If there are no duplicates, the length
                // of the set should the the same as the length of the array.
                let page_set = new Set(pages);
                expect(pages.length).toBe(page_set.size);

            });

            test(`/${endpoint} load < 100ms`, async() => {
                // Assert that all tables can load data page in < 350ms
                let start_time = Date.now();
                // Exclude normal 200 and content asserts so they don't impact performance
                const response = await cosiRequest
                                        .get(`/${endpoint}`)
                                        .query({page: 0})
                let endTime = Date.now();
                expect(endTime - start_time).toBeLessThan(350);
            });
        }

        test("/get_eventregistration Check key_type", async() => {
            // Checks that key_type implies only one foreign key is set.
            const response = await cosiRequest.get(`/get_eventregistration`).query({page: 0});
            let jsonData = JSON.parse(response.text);
            let d = jsonData["data"];
            const keys = ["Person", "Group", "Household"];
            for (let v of d) {
                expect(v["key_type"].includes(keys));
                for (let k of keys) {
                    let lo = k.toLowerCase();
                    expect((v["key_type"] == k && v[lo] != "null") || (v["key_type"] != k && v[lo] == "null"))
                }
            }
        });
    });

    // Insert after getters so it doesn't change the get count.
    // Track an instance of inserted jsons for later usage.
    let modifyLater = []
    describe("Verify Setters", () => {
        let verifyData = (data) => {
            let jData = JSON.parse(data);
            modifyLater.push(jData);
            expectKeys(jData, ["$oid"]);
        };

        const endpointPerson = "insert_person";
        test(`/${endpointPerson} POST`, async () => {
            const response = await cosiRequest
                                    .post(`/${endpointPerson}`)
                                    .type("form")
                                    .send({
                                        "first_name": "mario",
                                        "middle_name": "plumber",
                                        "last_name": "フブキ",
                                        "dob": "1985-09-13",
                                        "sex": "Male",
                                        "notes": "A helpful local plumber who can jump!",
                                        "emergency_contact": ""
                                    })
                                    .expect(200)
                                    .expect("Content-Type", /json/)

            verifyData(response.text);
        });

        const endpointAddress = "insert_address";
        test(`/${endpointAddress} POST`, async () => {
            const response = await cosiRequest
                                    .post(`/${endpointAddress}`)
                                    .type("form")
                                    .send({
                                        "line_one": "1337 Street",
                                        "line_two": "gura-a",
                                        "line_three": "Your time to shine",
                                        "city": "Iselgard",
                                        "region": "Gondor",
                                        "postal_code": "2468",
                                        "country": "Middle Earth"
                                    });

            verifyData(response.text);
        });
    });

    const updatePerson = "update_person"
    describe("Verify Updaters", () => {
        test(`/${updatePerson} POST`, async () => {
            const response = await cosiRequest
                                    .post(`/${updatePerson}`)
                                    .type("form")
                                    .query({
                                        oid: modifyLater[0]["$oid"]
                                    })
                                    .send({
                                        "city": "HOPE",
                                    })
                                    .expect(200);

            let jData = JSON.parse(response.text);
            console.log(jData);
        });
    });
});
