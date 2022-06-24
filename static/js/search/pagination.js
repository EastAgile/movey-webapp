class Pagination {
    constructor(currentPage, totalPages) {
        this.currentPage = currentPage;
        this.totalPages = totalPages;
        this.init();
    }

    init() {
        this.displayPages();
    }

    displayPages() {
        if (this.currentPage == 1) {
            $(".paginationjs-prev").remove();
        }

        let pageTemplate = $(".paginationjs-page");
        let ellipsisTemplate = $(".paginationjs-ellipsis");
        let paginationPage = pageTemplate;
        let totalPages = this.totalPages;
        let prevPageIdx = 0;

        // display at most 7 pages
        let displayPageNums = [
            1, 2,
            this.currentPage - 1, this.currentPage, this.currentPage + 1,
            this.totalPages - 1, this.totalPages
        ].filter(function(value, index, self) {
            return self.indexOf(value) === index && value >= 1 && value <= totalPages
        });

        for (let i=0; i<displayPageNums.length; i++) {
            let currPageIdx = displayPageNums[i];

            if (currPageIdx != prevPageIdx + 1) {
                console.log(currPageIdx, prevPageIdx);
                let ellipsis = ellipsisTemplate.clone();
                paginationPage.after(ellipsis);
                paginationPage = ellipsis;
            }

            let nextPage = pageTemplate.clone();

            if (currPageIdx == this.currentPage) {
                nextPage.addClass("active");
            }

            nextPage.attr("data-page", currPageIdx);
            nextPage.append("<a>" + currPageIdx.toString() + "</a>");
            paginationPage.after(nextPage);
            paginationPage = nextPage;
            prevPageIdx = currPageIdx;
        }

        pageTemplate.remove();
        ellipsisTemplate.remove();

        if (this.currentPage == this.totalPages) {
            $(".paginationjs-next").remove();
        }
    }
}