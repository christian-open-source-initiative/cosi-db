import request from 'supertest';

import { ALL_PAGEABLE_ENDPOINTS, ALL_GEN_ENDPOINTS } from "./endpoints.js";

// Basic helpers
function cosi_request() {
    return request("127.0.0.1:8000");
}

function expectKeys(json_data, keys) {
    expect(Object.keys(json_data)).toEqual(keys);
}

// Test setup
beforeAll(async ()=> {
    // Before tests begin, populate table with values.
    let total_data_points_per_table = 200;
    for (let endpoints of ALL_GEN_ENDPOINTS) {
        let response = await cosi_request().get(`/${endpoints}/${total_data_points_per_table}`)
                                           .expect(200)
                                           .expect("Content-Type", /json/);

        let json_data = JSON.parse(response.text);
        expectKeys(json_data, ["total"]);
        expect(json_data["total"]).toBe(total_data_points_per_table)
    }
});
const endpoint = "get_household";

// Testing
describe("Test Root", () => {
    test("/ GET", async () => {
        cosi_request().get(`/${endpoint}`).expect(200).expect("Content-Type", /json/);
    })
});

