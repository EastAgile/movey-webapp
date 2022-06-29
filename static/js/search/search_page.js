class SearchPage {
    constructor() {
        this.packagesContainerElement = $("#package-list-container");
        this.sortElement = $("select[name='packages-sort']");
        this.init();
    }

    init() {
        this.sortElement.select2({
            width: '100%',
            minimumResultsForSearch: -1,
            dropdownCssClass: 'packages-sort-dropdown'
        });
        this.sortElement.val(this.sortElement.data('sort')).trigger('change');

        this.sortElement.on('change', function(e) {
            const $elem = $(e.currentTarget);
            window.location = $elem.data('url') + '&field=' + $elem.val();
        });

        this.packagesContainerElement.on('click', '.paginationjs-page', (e) => {
            const $elem = $(e.currentTarget);
            window.location = $elem.data('url') + '&page=' + $elem.data('page');
        });

        this.packagesContainerElement.on('click', '.paginationjs-prev', (e) => {
            const $activePage = this.packagesContainerElement.find('.paginationjs li.active');
            $activePage.prev().click();
        })

        this.packagesContainerElement.on('click', '.paginationjs-next', (e) => {
            const $activePage = this.packagesContainerElement.find('.paginationjs li.active');
            $activePage.next().click();
        })

        $('.right-wrapper time').timeago();
    }
}
