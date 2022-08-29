const ALL_PAGEABLE_ENDPOINTS = [
    "get_person",
    "get_address",
    "get_household",
    "get_group",
    "get_grouprelation",
    "get_event",
    "get_eventregistration"
];

// Should be in order of least-dependent.
const ALL_GEN_ENDPOINTS = [
    "gen_person",
    "gen_address",
    "gen_household",
    "gen_group",
    "gen_grouprelation",
    "gen_event",
    "gen_eventregistration",
    "gen_login"
];

// All available endpoints in program.
const ENDPOINTS = ALL_GEN_ENDPOINTS + ALL_PAGEABLE_ENDPOINTS;

export {
    ALL_PAGEABLE_ENDPOINTS,
    ALL_GEN_ENDPOINTS,
    ENDPOINTS
}
