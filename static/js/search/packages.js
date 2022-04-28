class Packages {
    constructor() {
        this.sortElement = $("select[name='packages-sort']");
        this.init();
    }

    init() {
        this.sortElement.select2({
            minimumResultsForSearch: -1,
            dropdownCssClass: 'packages-sort-dropdown'
        });
        this.sortElement.val(this.sortElement.data('sort')).trigger('change');

        this.sortElement.on('change', function(e) {
            const $elem = $(e.currentTarget);
            window.location = $elem.data('url') + '&sort_type=' + $elem.val();
        });
        $('.right-wrapper time').timeago();
        $('.right-wrapper time').prepend("Updated ");
    }
}