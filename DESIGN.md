#  COSI DB

## Design Specifications

A church is composed of many people.
Each person can be grouped by relations like a family tree.
The family tree can have relationships both past and present.

Groups form the basis of multiple relationships.

### Group Tables

#### Groups

Good for capturing co-eccentric ministry hubs.

```
groups {
    group_id: xxx,
    group_name: "Small Groups",
    group_description: "Our usual meetings."
}
```

#### Group Relations

```
group_relations {
    person_id: xxx,
    group_id: xxx,
    role: "member"
}
```

#### Events Table

A group of people can meet at specific times and events.

```
events {
    meeting_days: [0, 1, 3],
    start_time: date
    end_time: date | NA
    start_date: date
    end_date: date
    reoccuring: Bool
}
```

#### Household

Captures the family dynamic.

```
household {
    house_name: xxx
    group_id: xxx
    persons: [xxx, yyy]
}
```

