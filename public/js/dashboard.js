// Selects which table to display
let ADDRESS_TABLE_IDX = 0;
let PEOPLE_TABLE_IDX = 1;
let HOUSEHOLD_TABLE_IDX = 3;
let GROUP_TABLE_IDX = 4;
let EVENT_TABLE_IDX = 5;
let TABLE_IDX = [
    ADDRESS_TABLE_IDX,
    PEOPLE_TABLE_IDX,
    HOUSEHOLD_TABLE_IDX,
    GROUP_TABLE_IDX,
    EVENT_TABLE_IDX
];
// Init tracker to default value.
let tableTrack = PEOPLE_TABLE_IDX;

let ENDPOINT = {};
ENDPOINT[ADDRESS_TABLE_IDX] = "address";
ENDPOINT[PEOPLE_TABLE_IDX] = "person";
ENDPOINT[HOUSEHOLD_TABLE_IDX] = "household";
ENDPOINT[GROUP_TABLE_IDX] = "group";
ENDPOINT[EVENT_TABLE_IDX] = "event";


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

let CURRENT_PAGE = 0;

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

    let tblClick = $("#table-left-move-click");
    let tbrClick = $("#table-right-move-click");
    let tbClicks = [tblClick, tbrClick];

    let tblBackground = $("#table-left")
    let tbrBackground = $("#table-right")
    let tbBackground = [tblBackground, tbrBackground];
    let incrementer = [-1, 1];

    // Table navigation logic.
    for (let i = 0; i < tbClicks.length; ++i) {
        tbClicks[i].on("mouseover", function() {
            tbBackground[i].css("background-color", "#3d526e");
        });
        tbClicks[i].on("mouseout", function() {
            tbBackground[i].css("background-color", "#1b2430");
        });
        tbClicks[i].on("click", function(e) {
            e.preventDefault();
            CURRENT_PAGE += incrementer[i];
            updateTable(appendFilter="", page=CURRENT_PAGE);
        });

        tbClicks[i].hide();
    }

    // General setup.
    // Miniboard for adding and updating actions.
    let miniBoard = new MiniBoard(
        $("#miniboard-render"),
        $("#cover-entire-screen-miniboard")
    );
    let actionToolbar = new ActionToolbar(miniBoard, $("#action-toolbar"));

    // Hide search suggestions until user inputs.
    let searchManager = new SearchManager(
        $("#main-search-bar"),
        $("#search-suggestions"),
        $("#main-search-bar-submit"),
        $("#cover-entire-screen-search")
    );

    let table = new Table(actionToolbar, $("#data-table"));

    // Logic to rerender the table by fetching data from endpoint.
    let updateTable = function(appendFilter = "", page=0) {
        // Update table.
        let fetchEndpoint = "/" + GET_ENDPOINT_LOOKUP[tableTrack] + `?page=${page}` + appendFilter;
        table.tableDiv.hide();

        let tName = ENDPOINT[tableTrack];
        $("#table-name").html(tName.charAt(0).toUpperCase() + tName.slice(1));
        $.get(fetchEndpoint, (result) => {
            table.render(tName, result["data"]);
            let totalPages = result["total_pages"]
            if (totalPages == 1) {
                tbrClick.hide();
                tblClick.hide();
            } else if (page + 1 == totalPages) {
                tbrClick.hide();
                tblClick.show();
            } else if (page == 0) {
                tblClick.hide();
                tbrClick.show();
            } else {
                tblClick.show();
                tbrClick.show();
            }

            let totalData = result["total_result"];
            if (totalPages == 0) {
                page = 0;
            } else {
                page += 1;
            }
            $(".page-count").html(`page ${page} of ${totalPages} (${result["data"].length} of ${totalData} rows total)`);
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

    $("#address-select").on("click", () => {
        tableTrack = ADDRESS_TABLE_IDX;
        CURRENT_PAGE = 0;
        actionToolbar.setState(new AddressState());
        updateTable();
    });

    $("#household-select").on("click", () => {
        tableTrack = HOUSEHOLD_TABLE_IDX;
        CURRENT_PAGE = 0;
        updateTable();
    });

    $("#people-select").on("click", () => {
        tableTrack = PEOPLE_TABLE_IDX;
        CURRENT_PAGE = 0;
        actionToolbar.setState(new PersonState());
        updateTable();
    });

    $("#group-select").on("click", () => {
        tableTrack = GROUP_TABLE_IDX;
        CURRENT_PAGE = 0;
        actionToolbar.setState(new GroupState());
        updateTable();
    });

    $("#event-select").on("click", () => {
        tableTrack = EVENT_TABLE_IDX;
        CURRENT_PAGE = 0;
        actionToolbar.setState(new EventState());
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
