class Header {
    constructor() {
        this.menu = $('.menu-nav');
        this.navLinks = $('.nav-links');
        this.signInButton = $('.sign-in');
        this.accountDropdownToggle = $('#account-dropdown-toggle');
        this.accountDropdownList = $('#account-dropdown-list');

        this.init();
    }

    init() {
        this.menu.click(() => {
            if (this.navLinks.css('left') === '0px') {
                this.navLinks.css('left', '-300px');
                this.signInButton.css('left', '-300px')
            } else {
                this.navLinks.css('left', '0px');
                this.signInButton.css('left', '0px')
            }
        })

        this.accountDropdownToggle.click(() => {
            if (this.accountDropdownList.css("display") === "none") {
                this.accountDropdownList.css('display', 'flex')
            } else {
                this.accountDropdownList.css('display', 'none')
            }
        })

        $(document).click(e => {
            let clicked = e.target.className;
            if(clicked !== 'nav-links' && clicked !== 'menu-nav') {
                this.navLinks.css('left', '-300px');
                this.signInButton.css('left', '-300px')
            }
        });
    }
}
