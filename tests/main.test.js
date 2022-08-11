import request from 'supertest';
import assert from 'assert';

import { ALL_PAGEABLE_ENDPOINTS, ALL_GEN_ENDPOINTS } from "./endpoints.js";

var total_datapoints_per_table = 200;

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
    for (let endpoints of ALL_GEN_ENDPOINTS) {
        let response = await cosi_request().get(`/${endpoints}/${total_datapoints_per_table}`)
                                           .expect(200)
                                           .expect("Content-Type", /json/);

        let json_data = JSON.parse(response.text);
        expectKeys(json_data, ["total"]);
        expect(json_data["total"]).toBe(total_datapoints_per_table)
    }
});

// Testing
describe("Test Root", () => {
    test("/ GET", async () => {
        cosi_request().get("/").expect(200);
    })
});

describe("Verify Getters", () => {
    // Check all basic GET endpoints.
    // Each get page should have max 100 datapoints.
    const max_datapoints = 100;
    const return_keys = [
        "page",
        "total_pages",
        "data"
    ];
    for (let endpoint of ALL_PAGEABLE_ENDPOINTS) {
        test(`/${endpoint} GET`, async () => {
            // Basic endpoint response verification.
            const response = await cosi_request()
                                    .get(`/${endpoint}`)
                                    .query({page: 0})
                                    .expect(200)
                                    .expect("Content-Type", /json/);

            // Further data verification.
            let json_data = JSON.parse(response.text);
            expectKeys(json_data, return_keys);
            expect(Object.keys(json_data["data"]).length).toBe(max_datapoints);
        });

        test(`/${endpoint} Empty page load`, async() => {
            const all_data = await cosi_request()
                                    .get(`/${endpoint}`)
                                    .expect(200)
                                    .query({page: Number.MAX_SAFE_INTEGER})
                                    .expect("Content-Type", /json/);
            let json_data = JSON.parse(all_data.text);
            expect(Object.keys(json_data["data"]).length).toBe(0);
        });

        test(`/${endpoint} Invalid page load`, async() => {
            const fake_pages = ["cosi", "-1", "!@#$%^&*()-_+=`"];
            for (const page of fake_pages){
                const all_data = await cosi_request()
                                    .get(`/${endpoint}`)
                                    .query({page: `${page}`})
                                    .expect(200)
                                    .expect("Content-Type", /json/);
                let json_data = JSON.parse(all_data.text);
                expect(json_data["page"]).toBe(0);
                expect(Object.keys(json_data["data"]).length).toBe(max_datapoints);
                expect(json_data["total_pages"]).toBe(Math.ceil(total_datapoints_per_table/max_datapoints));
            }
        });

        test(`/${endpoint} Correct page count`, async() => {
            const all_data = await cosi_request()
                                    .get(`/${endpoint}`)
                                    .expect(200)
                                    .expect("Content-Type", /json/);
            let json_data = JSON.parse(all_data.text);
            let total_pages = json_data["total_pages"];
            expect(total_pages).toBe(Math.ceil(total_datapoints_per_table/max_datapoints));

        });

        test(`/${endpoint} No duplicate page data`, async() => {
            let pages = [];
            // Concatenate all data to single array
            for(let page = 0; page < Math.ceil(total_datapoints_per_table/max_datapoints); page++){
                let request = await cosi_request()
                                    .get(`/${endpoint}`)
                                    .query({page: `${page}`})
                                    .expect(200)
                                    .expect("Content-Type", /json/)
                let json_data = JSON.parse(request.text);
                pages = pages.concat(Object.values(json_data["data"]));
            }
            // Converting to a set will de-duplicate data. If there are no duplicates, the length
            // of the set should the the same as the length of the array.
            let page_set = new Set(pages);
            expect(pages.length).toBe(page_set.size);

        });

        test(`/${endpoint} load < 100ms`, async() => {
            // Assert that all tables can load data page in < 100ms
            let start_time = Date.now();
            // Exclude normal 200 and content asserts so they don't impact performance
            const response = await cosi_request()
                                    .get(`/${endpoint}`)
                                    .query({page: 0})
            let end_time = Date.now();
            expect(end_time - start_time).toBeLessThan(100);
        });
    }
});

