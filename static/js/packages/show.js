class PackageShow {
    constructor() {
        this.readmeElement = $('.package-readme-content');
        this.instructionElement = $('.package-install-instruction');
        this.packageSizeElement = $('.package-size');
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

        this.packageSizeElement.text(this.niceBytes(this.packageSizeElement.data("value")));
    }

    // https://www.codegrepper.com/code-examples/javascript/kb+to+mb+to+gb+jquery
    niceBytes(x) {
        const units = ['bytes', 'KB', 'MB', 'GB', 'TB', 'PB', 'EB', 'ZB', 'YB'];
        let l = 0, n = parseInt(x, 10) || 0;
        while(n >= 1024 && ++l){
            n = n/1024;
        }
        return(n.toFixed(n < 10 && l > 0 ? 1 : 0) + ' ' + units[l]);
    }
}
