class SearchBar {
    constructor() {
        this.searchBtn = $('.search-btn');
        this.init();
    }

    init() {
        this.searchBtn.on('click', function(e) {
            let $searchBtn = $(e.currentTarget);
            let $searchBtnIcon = $('#search-btn-icon');
            let $searchBar = $('#search-bar');

            if ($searchBtn.data('active')) {
                $searchBtnIcon.addClass("fa-search");
                $searchBtnIcon.removeClass("fa-close");
                $searchBtn.data("active", false);
                $searchBar.css("background", "none");
            }
            else {
                $searchBtnIcon.addClass("fa-close");
                $searchBtnIcon.removeClass("fa-search");
                $searchBtn.data("active", true);
                $searchBar.css("background", "rgba(2, 20, 58, 0.8)");
            }
            $searchBtn.toggleClass("search-btn-active");
            $searchBar.toggle();
        });

        $('.fa-times-circle').first().on('click', function(e) {
            $('#search-field').val("");
        });
    }
}
