class Header {
    constructor() {
        this.searchIconElement = $('header.dark .search_icon');
        this.searchOverlay = $('header.dark .header-search-overlay');
        this.closeSearchOverlay = this.searchOverlay.find('.header-search-overlay-close');
        this.init();
    }

    init() {
        this.searchIconElement.on('click', (e) => {
            this.searchOverlay.toggle();
            this.searchOverlay.find('#header-search-input').focus();
        });

        this.closeSearchOverlay.on('click', (e) => {
            this.searchOverlay.hide();
        });
    }
}
