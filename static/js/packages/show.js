class PackageShow {
    constructor() {
        this.readmeElement = $('.package-readme-content');
        this.instructionElement = $('.package-install-instruction');
        this.init();

       
    }

    init() {
        var converter = new showdown.Converter();
        this.readmeElement.html(converter.makeHtml(this.readmeElement.html()));

        this.instructionElement.on('click', (e) => {
            navigator.clipboard.writeText(this.instructionElement.find('.instruction-command').text());
            $('.copy-tooltip').show();
            setTimeout(() => {
                $('.copy-tooltip').hide();
            }, 400);
        });
    }
}
