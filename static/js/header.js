class Header {
    constructor() {
        this.navLinks = $('.nav-links');

        this.init();
    }

    init() {

        $(document).click(e => {
            let clicked = e.target.className;
            if(clicked !== 'nav-links') {
                this.navLinks.css('left', '-300px');
            }
        });
    }
}
