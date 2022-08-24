class Header {
    constructor() {
        this.navLinks = $('.nav-links');
        this.accountDropdownToggle = $('#account-dropdown-toggle');
        this.accountDropdownList = $('#account-dropdown-list');
        this.logoutForm = $('.logout-form');
        this.init();
    }

    init() {
        this.checkForLoggedInUser()

        this.accountDropdownToggle.click(() => {
            if (this.accountDropdownList.css("display") === "none") {
                this.accountDropdownList.css('display', 'flex')
                this.accountDropdownList.css('justify-content', 'flex-start')
            } else {
                this.accountDropdownList.css('display', 'none')
            }
        })

        this.logoutForm.find('a').on('click', () => {
            this.logoutForm.submit()
        })
    }

    checkForLoggedInUser() {
        $.ajax({
            method: 'GET',
            url: '/api/v1/me',
            success: (data) => {
                $(".header-container .search-li").removeClass('hide');
                if(data.id) {
                    $(".header-container #account-dropdown").removeClass('hide');
                    const char = (data.name != '' ? data.name[0] : data.email[0]);
                    // add image in header
                    $(".header-container #account-dropdown #account-icon").text("");
                    var x = document.createElement("img");
                    x.setAttribute("src", data.avatar);
                    x.setAttribute("width", "40");
                    x.setAttribute("height", "40");
                    x.setAttribute("id","account-icon");
                    x.setAttribute("alt", "Avatar of Movey website");
                    $(".header-container #account-dropdown #account-icon").append(x);
                    $(".search-btn").addClass('log');
                } else {
                    $(".header-container .sign-in-li").removeClass('hide');
                    $(".header-container .sign-up-li").removeClass('hide');
                }
            },
            error: () => {
                $(".header-container .sign-in-li").removeClass('hide');
                $(".header-container .sign-up-li").removeClass('hide');
            },
        })
    }
}
