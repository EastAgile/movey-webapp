class SearchBar {
    constructor() {
        this.searchBtn = $('.search-btn');
        this.searchBtnIcon = $('#search-btn-icon');
        this.searchBar = $('#search-bar');
        this.searchForm = $('#search-bar form');
        this.searchField = $('#search-field');
        this.init();
    }

    openSearchModal() {
        this.searchBtnIcon.addClass("fa-close");
        this.searchBtnIcon.removeClass("fa-search");
        this.searchBtn.data("active", true);
        this.searchBar.css("background", "rgba(2, 20, 58, 0.8)");
        this.searchBar.ready(() => { 
            this.searchField.focus();
        })

        this.searchBtn.toggleClass("search-btn-active");
        this.searchBar.toggle();
    }

    closeSearchModal() {
        this.searchBtnIcon.addClass("fa-search");
        this.searchBtnIcon.removeClass("fa-close");
        this.searchBtn.data("active", false);
        this.searchBar.css("background", "none");
    
        this.searchBtn.toggleClass("search-btn-active");
        this.searchBar.toggle();
    }

    init() {
        this.searchBtn.on('click', (e) => {
            $(e.currentTarget).data('active') 
                ? this.closeSearchModal()
                : this.openSearchModal();
        });

        this.searchForm.on('click', (e) => {
            this.searchField.focus();
        })

        $('.fa-times-circle').first().on('click', function(e) {
            $('#search-field').val("");
        });

        $(document).mouseup((e) => {
            if (e.target.id === "search-bar" || e.target.tagName === "NAV") {
                this.closeSearchModal();
            }
        });
    }
}
