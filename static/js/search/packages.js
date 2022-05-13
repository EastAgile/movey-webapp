class Packages {
    constructor(packages) {
        this.sortElement = $("select[name='packages-sort']");
        this.packages = JSON.parse(packages.replace(/&quot;/g, '\"'));
        this.pageSize = 20;
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
            window.location = $elem.data('url') + '&field=' + $elem.val();
        });
        
        $('#package-list-container').pagination({
            dataSource: this.packages,
            pageSize: this.pageSize,
            showNavigator: true,
            autoHidePrevious: true,
            autoHideNext: true,
            prevText: "<i class='fa fa-chevron-left'></i>",
            nextText: "<i class='fa fa-chevron-right'></i>",
            formatNavigator: this.genNavText,
            callback: function(data, pagination) {
                // template method of yourself
                var html = Packages.template(data);
                $('.package-list').html(html);
                $('.right-wrapper time').timeago();    
            }
        });
    }

    genNavText(currentPage, totalPage, totalNumber) {
        let start = this.pageSize * (currentPage - 1) + 1;
        let end = 0;
        if (currentPage == totalPage) {
            end = totalNumber;
        }
        else {
            end = this.pageSize * currentPage;
        }
        return `Displaying ${ start }-${ end } of ${ totalNumber } total results`;
    }

    static template(packages) {
        let html = "";
        for (let p of packages) {
            html = html.concat(
                `<div class='package-list-item'>
                    <div class='left-wrapper'>
                        <div class='package-list-item-title'>
                            <h1>
                                <span>${ p.name }</span>&nbsp;
                                <span class='version-number'>${ p.version }</span>
                            </h1>
                            <div class='title-tag'>
                                1st tag
                            </div>
                        </div>
                        <div class='sub-tag-list'>
                            <div class='sub-tag'>Normal Tag</div>
                            <div class='sub-tag'>Normal Tag</div>
                        </div>
                        <div class='package-summary'>
                            ${ p.description }
                        </div>
                    </div>
                    <div class='right-wrapper'>
                        <div class='download-count'>
                            <img src='/static/resources/download_icon.svg'>
                            <b>${ p.total_downloads_count }</b>
                        </div>
                        <div class='update-timestamp'>
                            <img src='/static/resources/reload_icon.svg'>
                            <time datetime='${ p.updated_at }'>${ p.updated_at }</time>
                        </div>
                    </div>
                </div>
                `
            );
        }
        return html;
    }
}