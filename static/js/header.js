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
                if(data.id) {
                   $(".header-container .sign-in-li").addClass('hide');
                   $(".header-container .sign-up-li").addClass('hide');
                   $(".header-container #account-dropdown").removeClass('hide');
                    
                   const char = (data.name != '' ? data.name[0] : data.email[0]);
                   $(".header-container #account-dropdown #account-icon").text(char);
                   $(".search-btn").addClass('log');
                }
            }
        })
    }
}
