// Certain tables have foreign key references that need rendering.
// This tracks what to display as clickable in those values.
let FOREIGN = {};
FOREIGN["household"] = {
    "persons": ["first_name", "last_name"],
    "address": ["line_one", "line_two", "line_three"]
};

// Special header renames to make things easier to read.
let RENAME = {
    "emergency_contact": "emergency",
    "first_name": "first",
    "middle_name": "middle",
    "last_name": "last",
    "home_phone": "home",
    "work_phone": "work",
    "mobile_phone": "mobile"
}

class Table {
    constructor(actionToolbar, tableDiv) {
        this.tableDiv = tableDiv
        this.actionToolbar = actionToolbar;

        let prevRef = undefined;
        $("#data-table").on("click", "tr", function() {
            if (prevRef !== undefined) {
                prevRef.removeClass("hover-click-highlight");
            }
            let tds = $(`[oid=${$(this).attr("oid")}] td`)
            tds.addClass("hover-click-highlight");
            prevRef = tds;
        });
    }

    render(tableName, data) {
        const displaySpeed = 1000;
        this.tableDiv.hide().empty();
        if (data.length == 0) {
            this.tableDiv.html("No Data!");
            this.tableDiv.show(1000);
            return;
        }

        // Header generate.
        let headerRow = $("<thead>");
        this.tableDiv.append(headerRow);
        let keys = Object.keys(data[0]);
        for (let h = 0; h < keys.length; ++h) {
            if (keys[h] == "_id") { continue; }
            let rename = keys[h] in RENAME ? RENAME[keys[h]] : keys[h];
            headerRow.append($("<th>").html(rename));
        }

        // Body generate.
        let foreignKeys = tableName in FOREIGN ? FOREIGN[tableName]: {}; // Can be undefined.
        for (let i = 0; i < data.length; ++i) {
            let row = this.tableDiv[0].insertRow(-1);
            let oid = undefined;

            for (let h = 0; h < keys.length; ++h) {
                let k = keys[h];
                let value = data[i][k];
                if (k == "_id") {
                    oid = value["$oid"];
                    continue;
                } else if (k in foreignKeys) {
                    let externalKeys = foreignKeys[k]
                    let extValue = value;

                    let retrieve = (keys, d) => {
                        let result = [];
                        for (let k of keys) {
                            result.push(d[k])
                        }
                        return result.join(" ");
                    };

                    let finalRender = "";
                    if (Array.isArray(extValue)) {
                        let results = [];
                        for (let ev of extValue) {
                            results.push(retrieve(externalKeys, ev));
                        }
                        finalRender = results.join(", ");
                    } else {
                        finalRender = retrieve(externalKeys, extValue);
                    }

                    $(row.insertCell(-1)).html(finalRender).attr("entry-name", k);
                } else {
                    if (Array.isArray(value)) {
                        $(row.insertCell(-1)).html(JSON.stringify(value)).attr("entry-name", k);
                    } else {
                        $(row.insertCell(-1)).html(decodeURIComponent(value)).attr("entry-name", k);
                    }
                }
            }

            $(row).attr("oid", oid);
        }
        this.actionToolbar.showButtons();
        this.tableDiv.show(displaySpeed);
    }
}
