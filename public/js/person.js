// Selects which table to display
let CURRENT_PAGE = 0;

// Logic dealing with the search function.
$(document).ready(() => {
    // Scroll effects.
    // Loading bar hooks.
    let loading =  $("#loading").hide();

    $(document).ajaxStart(() => {
        loading.show();
    }).ajaxStop(()=>{
        loading.hide();
    });
});
