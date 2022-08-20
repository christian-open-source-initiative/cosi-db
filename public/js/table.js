class Table {
    constructor(tableDiv) {
        this.tableDiv = tableDiv

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

    render(data) {
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
            headerRow.append($("<th>").html(keys[h]));
        }

        // Body generate.
        for (let i = 0; i < data.length; ++i) {
            let row = this.tableDiv[0].insertRow(-1);
            let oid = undefined;

            for (let h = 0; h < keys.length; ++h) {
                let k = keys[h];
                if (k == "_id") { oid = data[i][k]["$oid"]; continue; }
                $(row.insertCell(-1)).html(data[i][k]).attr("entry-name", k);
            }

            $(row).attr("oid", oid);
        }
        this.tableDiv.show(displaySpeed);
    }
}
