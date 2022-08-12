class Table {
    constructor(tableDiv) {
        this.tableDiv = tableDiv
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
            headerRow.append($("<th>").html(keys[h]));
        }

        // Body generate.
        for (let i = 0; i < data.length; ++i) {
            let row = this.tableDiv[0].insertRow(-1);

            for (let h = 0; h < keys.length; ++h) {
                let k = keys[h];
                if (k == "_id") {
                    $(row.insertCell(-1)).html(data[i][k]["$oid"]);
                }
                else {
                    $(row.insertCell(-1)).html(data[i][k]);
                }
            }
        }
        this.tableDiv.show(displaySpeed);
    }
}
