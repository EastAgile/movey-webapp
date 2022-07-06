class PackageVersions {
    constructor() {
        this.sortElement = $("select[name='versions-sort']");
        this.packageDescription = $('.package-description');
        this.init();
    }

    init() {
        var converter = new showdown.Converter();
        this.packageDescription.html(converter.makeHtml(this.packageDescription.html()));

        this.sortElement.select2({
            width: '100%',
            minimumResultsForSearch: -1,
            dropdownCssClass: 'versions-sort-dropdown'
        });
        this.sortElement.val(this.sortElement.data('sort')).trigger('change');

        this.sortElement.on('change', function(e) {
            const $elem = $(e.currentTarget);
            window.location = $elem.data('url') + '?sort_type=' + $elem.val();
        });

        $('.package-version-item time').timeago();
    }
}
