class Header {
    constructor() {
        this.searchIconElement = $('header.dark .search_icon');
        this.searchOverlay = $('header.dark .header-search-overlay');
        this.closeSearchOverlay = this.searchOverlay.find('.header-search-overlay-close');
        this.menu = $('.menu-nav');
        this.navLinks = $('.nav-links');
        this.signInButton = $('.sign-in');

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

        this.menu.click(() => {
            if (this.navLinks.css('left') === '0px') {
                this.navLinks.css('left', '-300px');
                this.signInButton.css('display', 'none')
            } else {
                this.navLinks.css('left', '0px');
                this.signInButton.css('display', 'block')
            }
        })

        $(document).click(e => {
            let clicked = e.target.className;
            if(clicked !== 'nav-links' && clicked !== 'menu-nav') {
                this.navLinks.css('left', '-300px');
                this.signInButton.css('display', 'none')
            }
        });
    }
}
