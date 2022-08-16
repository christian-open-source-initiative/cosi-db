$(document).ready(function() {
    $("#login-form").on("submit", function(h) {
        h.preventDefault();
        $.ajax({
            url: $(this).attr("action"),
            type: "POST",
            data: $(this).serialize(),
            success: function(data) {
                $("#auth-status").hide();
                let text = Object.values(data)[0];
                if ("err" in data) {
                    text = `<div class="err"> ${text} </div>`
                }
                else {
                    text = `<div class="success"> ${text} </div>`
                    setTimeout(() => {
                        $(window).attr("location", "/");
                    }, 500);
                }
                $("#auth-status").html(text);
                $("#auth-status").show(250);
            },
            error: (_, __, e) => {
                console.log(e);
            }
        });
    });
});
