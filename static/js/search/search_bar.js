
function toggleSearchDropDown(el) {
    let $search_btn = $(el);
    let $search_btn_icon = $("#search-btn-icon");
    let $search_bar = $("#search-bar");
    if ($search_btn.data('active')) {
        $search_btn_icon.addClass("fa-search");
        $search_btn_icon.removeClass("fa-close");
        $search_btn.data("active", false);
        $search_bar.css("background", "none");
    }
    else {
        $search_btn_icon.addClass("fa-close");
        $search_btn_icon.removeClass("fa-search");
        $search_btn.data("active", true);
        $search_bar.css("background", "rgba(2, 20, 58, 0.8)");
    }
    $search_btn.toggleClass("search-btn-active");
    $search_bar.toggle();
}

function clearSearchField() {
    $("#search-field").val("");
}