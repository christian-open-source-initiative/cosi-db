let updateTable = function() {
    let table = $("#data-table");
    table.empty().hide();
    $.get("/get_people?page=0", (data) => {
        let actualData = data["data"];
        if (actualData.length == 0) {
            table.html("No Data!");
            return;
        }

        // Headers
        let header_row = $("<thead>");
        table.append(header_row);
        let keys = Object.keys(actualData[0]);
        for (let h = 0; h < keys.length; ++h) {
            header_row.append($("<th>").html(keys[h]));
        }
        for (let i = 0; i < actualData.length; ++i)
        {
            let row = table[0].insertRow(-1);

            for (let h = 0; h < keys.length; ++h) {
                $(row.insertCell(-1)).html(actualData[i][keys[h]]);
            }
        }

        table.show(1000);
    });

};

$(document).ready(() => {
    // Loading bar hooks.
    let loading =  $("#loading").hide();

    $(document).ajaxStart(() => {
        loading.show();
    }).ajaxStop(()=>{
        loading.hide();
    });

    // Generate data action.
    let generate_total = 200;
    $("#gen-people").click(() => {
        $.get("/gen_people/" + generate_total + "/", (data) => {
            console.log(data);
            $("#status").hide().html("Generated total datapoints: " + data["total"]).show();
            updateTable();
        })
        .fail((d, textStatus, error) => {console.log(error);});
    });

});
