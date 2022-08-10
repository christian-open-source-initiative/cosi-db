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

// Maps which keys to search for a given endpoint.
let ENDPOINT_SEARCHABLE = {};
ENDPOINT_SEARCHABLE[ENDPOINT[ADDRESS_TABLE_IDX]] = ["line_one", "line_two", "line_three"];
ENDPOINT_SEARCHABLE[ENDPOINT[PEOPLE_TABLE_IDX]] = ["first_name", "middle_name"];
ENDPOINT_SEARCHABLE[ENDPOINT[HOUSEHOLD_TABLE_IDX]] = ["house_name"]

// End point mapping
let GEN_ENDPOINT_LOOKUP = {};
let GET_ENDPOINT_LOOKUP = {};
for (const idx of TABLE_IDX) {
    GET_ENDPOINT_LOOKUP[idx] = "get_" + ENDPOINT[idx];
    GEN_ENDPOINT_LOOKUP[idx] = "gen_" + ENDPOINT[idx];
}

class SearchManager {
    constructor(searchBar, searchSuggestion, searchButton, searchDarkener, tablesSearch=ENDPOINT) {
        this.searchBar = searchBar;
        this.searchSuggestion = searchSuggestion;
        this.searchButton = searchButton;
        this.searchDarkener = searchDarkener;

        // Initial states
        this.searchSuggestion.hide();
        this.searchDarkener.hide();
        // We don't want to spam the server
        this.currentQuery = "";
        this.tablesSearch = {...tablesSearch};
        // Stores tables that are valid to be searched.
        this.validSearches = {...ENDPOINT_SEARCHABLE};

        // Requires binding due to function reference.
        this.searchBar.keyup(
            (event) => {
                if (event.key == "Escape") {
                    this.searchDarkener.hide();
                    // Unfocus so that we can refocus if start typing.
                    this.searchDarkener.blur();
                    return;
                }
                this.determineHide();
                if (this.currentQuery.length != this.searchBar.val().length) {
                    this.currentQuery = this.searchBar.val();
                    this.dispatchSearch();
                }
            }
        );

        // We only want to hid if user focuses and already typed.
        this.searchBar.focus(this.determineHide.bind(this));
        this.searchBar.blur(() => { this.searchDarkener.hide(); });
    }

    dispatchSearch() {
        // Dispatches the search result to all available tables.
        this.searchSuggestion.empty();
        for (const tableName in this.validSearches) {
            let idx = R_ENDPOINT[tableName];
            let endpoint = "/" + GET_ENDPOINT_LOOKUP[idx] + "/";
            let queries = this.validSearches[tableName];

            for (let q of queries) {
                $.get(endpoint + "/?page=0" + `&${q}=${this.currentQuery}`, (data) => {
                    if (data["data"].length != 0) {
                        console.log(data["data"]);
                        this.searchSuggestion.append(data["data"]);
                    }
                })
                .fail((d, textStatus, error) => {console.log(error);});
            }
        }
    }

    determineHide() {
        if (this.searchBar.val() == "") {
            this.searchDarkener.hide();
            return true;
        }

        this.searchDarkener.show();
    }
}

function generalSetup() {
    // Hide search suggestions until user inputs.
    let searchManager = new SearchManager(
        $("#main-search-bar"),
        $("#search-suggestions"),
        $("#main-search-bar-submit"),
        $("#cover-entire-screen")
    );

}

// Logic dealing with the search function.
$(document).ready(() => {
    generalSetup();

    // Generate data action.
    let generateTotal = 200;
    $("#gen-data").click(() => {
        // Resolve the appropriate endpoint depending on the state
        // of the buttons pressed.
        let endpoint = "/" + GEN_ENDPOINT_LOOKUP[tableTrack] + "/";
        $.get(endpoint + generateTotal + "/", (data) => {
            $("#status").hide().html("Generated total datapoints: " + data["total"]).show();
            updateTable();
        })
        .fail((d, textStatus, error) => {console.log(error);});
    });


    // Logic to rerender the table by fetching data from endpoint.
    let updateTable = function() {
        let table = $("#data-table");
        table.empty().hide();

        // Update table.
        let fetchEndpoint = "/" + GET_ENDPOINT_LOOKUP[tableTrack] + "?page=0";
        $.get(fetchEndpoint, (data) => {
            let actualData = data["data"];
            if (actualData.length == 0) {
                table.html("No Data!");
                return;
            }

            // Headers
            let headerRow = $("<thead>");
            table.append(headerRow);
            let keys = Object.keys(actualData[0]);
            for (let h = 0; h < keys.length; ++h) {
                headerRow.append($("<th>").html(keys[h]));
            }
            for (let i = 0; i < actualData.length; ++i) {
                let row = table[0].insertRow(-1);

                for (let h = 0; h < keys.length; ++h) {
                    $(row.insertCell(-1)).html(actualData[i][keys[h]]);
                }
            }

            // Show with smooth transition delay.
            table.show(1000);
        });

    };

    // Register callbacks.
    $("#address-select").click(() => {
        tableTrack = ADDRESS_TABLE_IDX;
        updateTable();
    });

    $("#household-select").click(() => {
        tableTrack = HOUSEHOLD_TABLE_IDX;
        updateTable();
    });

    $("#people-select").click(() => {
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
