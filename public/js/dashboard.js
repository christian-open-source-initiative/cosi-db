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
    // Scroll effects.
    var screenHeight = $(window).height();
    let tbNav= $("#table-left-arrow,#table-right-arrow,#table-left,#table-right");
    const startPercent = 80;
    tbNav.css({"top": `${startPercent}%`});
    $(window).scroll(function() {
        let threshold = 0.5 * screenHeight;
        if ($(window).scrollTop() > threshold) {
            tbNav.css({"top": "50%"});
        }
        else {
            let mPercent = startPercent - (startPercent - 50) * $(window).scrollTop() / threshold;
            tbNav.css({"top": `${mPercent}%`});
        }
    });

    let tbClick = $(".table-move-click");
    let tbBackground = $("#table-left,#table-right")
    tbClick.on("mouseover", function() {
       tbBackground.css("background-color", "#3d526e");
    });
    tbClick.on("mouseout", function() {
       tbBackground.css("background-color", "#1b2430");
    });


    // General setup.
    // Hide search suggestions until user inputs.
    let searchManager = new SearchManager(
        $("#main-search-bar"),
        $("#search-suggestions"),
        $("#main-search-bar-submit"),
        $("#cover-entire-screen")
    );

    let table = new Table($("#data-table"));

    // Logic to rerender the table by fetching data from endpoint.
    let updateTable = function(appendFilter = "") {
        // Update table.
        let fetchEndpoint = "/" + GET_ENDPOINT_LOOKUP[tableTrack] + "?page=0" + appendFilter;
        table.tableDiv.hide();

        let tName = ENDPOINT[tableTrack];
        $("#table-name").html(tName.charAt(0).toUpperCase() + tName.slice(1));
        $.get(fetchEndpoint, (result) => {
            table.render(result["data"]);
        });
    };

    // Register callbacks.
    // TODO: Not sure how to encapsulate this when ownership of table state is elsewhere.
    // Reaction to clicking search suggestions.
    $("#search-suggestions").on("click", ".search-suggestion-entry", function() {
        // TODO: Placeholder for now until we can scroll to result. Regenerate table.
        let fullMatch = $(this).children(".search-suggestion-result").attr("data");
        let entry = $(this).children(".search-suggestion-table").attr("entry");
        let tableName = $(this).children(".search-suggestion-table").attr("table");

        tableTrack = R_ENDPOINT[tableName.toLowerCase()];
        updateTable(`&${entry}=${fullMatch}`);
        $("#main-search-bar").val("");
        searchManager.determineHide();
    });

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
