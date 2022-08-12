// Selects which table to display
let ADDRESS_TABLE_IDX = 0;
let PEOPLE_TABLE_IDX = 1;
let HOUSEHOLD_TABLE_IDX = 3;
let TABLE_IDX = [
    ADDRESS_TABLE_IDX,
    PEOPLE_TABLE_IDX,
    HOUSEHOLD_TABLE_IDX
];
// Init tracker to default value.
let tableTrack = PEOPLE_TABLE_IDX;

let ENDPOINT = {};
ENDPOINT[ADDRESS_TABLE_IDX] = "address";
ENDPOINT[PEOPLE_TABLE_IDX] = "person";
ENDPOINT[HOUSEHOLD_TABLE_IDX] = "household";

let R_ENDPOINT = {};
for (let k in ENDPOINT) {
    R_ENDPOINT[ENDPOINT[k]] = k;
}

// End point mapping
let GEN_ENDPOINT_LOOKUP = {};
let GET_ENDPOINT_LOOKUP = {};
for (const idx of TABLE_IDX) {
    GET_ENDPOINT_LOOKUP[idx] = "get_" + ENDPOINT[idx];
    GEN_ENDPOINT_LOOKUP[idx] = "gen_" + ENDPOINT[idx];
}

// Logic dealing with the search function.
$(document).ready(() => {
    // General setup.
    // Hide search suggestions until user inputs.
    let searchManager = new SearchManager(
        $("#main-search-bar"),
        $("#search-suggestions"),
        $("#main-search-bar-submit"),
        $("#cover-entire-screen")
    );

    let table = new Table($("#data-table"));

    // Generate data action.
    let generateTotal = 200;
    $("#gen-data").on("click", () => {
        // Resolve the appropriate endpoint depending on the state
        // of the buttons pressed.
        let endpoint = "/" + GEN_ENDPOINT_LOOKUP[tableTrack] + "/";
        table.tableDiv.hide();
        $.get(endpoint + generateTotal + "/", (data) => {
            $("#status").hide().html("Generated total datapoints: " + data["total"]).show();
            updateTable();
        })
        .fail((d, textStatus, error) => {console.log(error);});
    });

    // Logic to rerender the table by fetching data from endpoint.
    let updateTable = function() {
        // Update table.
        let fetchEndpoint = "/" + GET_ENDPOINT_LOOKUP[tableTrack] + "?page=0";
        table.tableDiv.hide();
        $.get(fetchEndpoint, (result) => {
            table.render(result["data"]);
        });
    };

    // Register callbacks.
    $("#address-select").on("click", () => {
        tableTrack = ADDRESS_TABLE_IDX;
        updateTable();
    });

    $("#household-select").on("click", () => {
        tableTrack = HOUSEHOLD_TABLE_IDX;
        updateTable();
    });

    $("#people-select").on("click", () => {
        tableTrack = PEOPLE_TABLE_IDX;
        updateTable();
    });

    // Loading bar hooks.
    let loading =  $("#loading").hide();

    $(document).ajaxStart(() => {
        loading.show();
    }).ajaxStop(()=>{
        loading.hide();
    });
});
