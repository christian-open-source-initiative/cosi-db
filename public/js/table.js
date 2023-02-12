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

// Foreign keys require special render constraints.
class CustomTableRender {
    setColumn(c) {
        this.column = c;
        return this;
    }

    setData(d) {
        this.data = d;
        return this;
    }

    render() {
        console.assert("No implemented err.")
    }
}

class BasicForeignRender extends CustomTableRender {
    constructor(externalKeyNames) {
        super();
        // Keys we want to render for foreign entry.
        this.keys = externalKeyNames;
        console.assert(Array.isArray(this.keys));
    }

    getKeysAsArray(keys, data) {
        let result = [];
        keys.forEach(k => result.push(data[k]));
        return result;
    }

    render(cb, fcb) {
        // For multiple relations.
        if (Array.isArray(this.data)) {
            let results = [];
            this.data.forEach(datum => {
                let s = this.getKeysAsArray(this.keys, datum).join(", ");
                results.push(s)
            });
            return results.join(", ");
        }

        cb(this.getKeysAsArray(this.keys, this.data).join(", "));
    }
}

class HouseRelationRender extends CustomTableRender {
    render(cb, fcb) {
        // Serialize each request to a single call.
        // Ordering is guaranteed by the API so we easily deserialize later.
        let oids = [];
        let relations = [];
        console.log(this.data);
        this.data.forEach(d => {
            oids.push(d["person_a"]["$oid"]);
            oids.push(d["person_b"]["$oid"]);
            relations.push(d["relation"])
        });
        console.assert(relations.length * 2 == oids.length);
        console.log(oids);

        $.ajax({
            url: `/get_person`,
            data: {oids: oids},
            success: function(result) {
                /// TODO: Error handling.
                let results = [];
                relations.forEach((r, idx) => {
                    let personA = result[idx * 2].first_name;
                    // Same person means it is a global role.
                    let s = null;
                    if (result.length == 1) {
                        s = `(${personA}, ${r})`;
                    } else if (result.length == 2) {
                        let personB = result[idx * 2 + 1].first_name;
                        s = `(${personA}, ${personB}, ${r})`;
                    }

                    results.push(s);
                });

                cb(results.join(", "));
            }
        }).fail(fcb);
    }
}

// We provide special ways to render for these entries.
let SPECIAL_RENDER = {};
SPECIAL_RENDER["household"] = {
    "persons": new BasicForeignRender(["first_name", "last_name"]),
    "address": new BasicForeignRender(["line_one", "line_two", "line_three"]),
    "relations": new HouseRelationRender()
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
            let oid = $(this).attr("oid");
            let tds = $(`[oid=${oid}] td`)
            tds.addClass("hover-click-highlight");
            prevRef = tds;
            actionToolbar.setSelected(oid);
        });
    }

    render(tableName, data) {
        const displaySpeed = 200;
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
        let specialKeys = tableName in SPECIAL_RENDER ? SPECIAL_RENDER[tableName]: {};
        for (let i = 0; i < data.length; ++i) {
            let row = this.tableDiv[0].insertRow(-1);
            let oid = undefined;

            for (let h = 0; h < keys.length; ++h) {
                let k = keys[h];
                let value = data[i][k];
                if (k == "_id") {
                    oid = value["$oid"];
                    continue;
                } else if (k in specialKeys) {
                    let renderer = specialKeys[k];
                    // Due to non-block, we want to create the cell in advance.
                    let newCellRef = $(row.insertCell(-1));
                    let cb = (renderHtml) => {
                        newCellRef.html(renderHtml).attr("entry-name", k);
                    }
                    renderer.setData(value)
                            .setColumn(k)
                            .render(cb, () => {newCellRef.html("Error fetching data.")});
                } else {
                    if (Array.isArray(value)) {
                        $(row.insertCell(-1)).html(JSON.stringify(value)).attr("entry-name", k);
                    } else {
                        let v = "";
                        if (value != null) {
                            v = decodeURIComponent(value);
                        }
                        $(row.insertCell(-1)).html(v).attr("entry-name", k);
                    }
                }
            }

            $(row).attr("oid", oid);
        }
        this.actionToolbar.showButtons();
        this.actionToolbar.setSelected(null);
        this.tableDiv.fadeIn(displaySpeed);
    }
}
