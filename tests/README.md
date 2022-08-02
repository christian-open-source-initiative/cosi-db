# API Level Testing

`tests/` contain API level tests that query our application on an API level to verify that results are correct.

## Running Tests

```bash
npm test
```

## Dependencies

Uses node `16.16.0` which is LTS. See `package.json`. Test framework uses `jest` and `supertest` for reqest-level asserts.
We use ES6 notation supported by NodeJS `16.16.0`.

## Writing and Contributing to Tests


This is what an average test looks like:

```javascript
const endpoint = "/get_address";
const max_datapoints = 100;
test(`/${endpoint} GET`, async () => {
    // Basic endpoint response verification.
    const response = await cosi_request()
                            .get(`/${endpoint}`)
                            .query({page: 0})
                            .expect("Content-Type", /json/)
                            .expect(200);

    // Further data verification.
    let json_data = JSON.parse(response.text);
    expect(Object.keys(json_data["data"]).length).toBe(max_datapoints);
});
```

Some key things to note:

- As a rule of thumb, payload should be asserted by `supertest`
  - We check immediately for a `200` response.
  - We check immeidately for JSON content.
- Semantic level data should be asserted by `jest`
  - Here we check that the `data` field contains exactly 100 datapoints.
