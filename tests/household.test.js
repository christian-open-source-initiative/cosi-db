import request from 'supertest';

import { ALL_PAGEABLE_ENDPOINTS, ALL_GEN_ENDPOINTS } from "./endpoints.js";

// Basic helpers
function cosiRequest() {
    return request("127.0.0.1:8000");
}
const endpoint = "find_household";

// Testing
describe("Test Root", () => {
    test("/ GET", async () => {
        cosiRequest().get(`/${endpoint}`).expect(200).expect("Content-Type", /json/);
    })
});
