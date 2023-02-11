const TABLE_NAMES = [
    "Person",
    "Address",
    "Household",
    "Group",
    "GroupRelation",
    "Event",
    "EventRegistration"
];

let ALL_PAGEABLE_ENDPOINTS = [];
for (const v of TABLE_NAMES) {
    ALL_PAGEABLE_ENDPOINTS.push(`find_${v.toLowerCase()}`);
}

// Should be in order of least-dependent.
// Does not include generation of login as it should be a one-time call in setup.
let ALL_GEN_ENDPOINTS = [];
for (const v of TABLE_NAMES) {
    ALL_GEN_ENDPOINTS.push(`gen_${v.toLocaleLowerCase()}`);
}

// All available endpoints in program.
const ENDPOINTS = ALL_GEN_ENDPOINTS + ALL_PAGEABLE_ENDPOINTS;

export {
    ALL_PAGEABLE_ENDPOINTS,
    ALL_GEN_ENDPOINTS,
    TABLE_NAMES,
    ENDPOINTS
}
