class PackageShow {
    constructor() {
        this.readmeElement = $('.package-readme-content');
        this.instructionElement = $('.package-install-instruction');
        this.packageSizeElement = $('.package-size');
        this.packageDescription = $('.package-description');
        this.init();
    }

    init() {
        var converter = new showdown.Converter({
            'tables': 'true'
        });
        this.readmeElement.html(converter.makeHtml(this.readmeElement.text()));
        this.packageDescription.html(converter.makeHtml(this.packageDescription.html()));
        this.instructionElement.on('click', (e) => {
            navigator.clipboard.writeText(this.instructionElement.find('.instruction-command').text());
            $('.copy-tooltip').show();
            setTimeout(() => {
                $('.copy-tooltip').hide();
            }, 400);
        });
        Array.from($(".package-readme-content a")).forEach(e => e.setAttribute('target', '_blank'));
        this.packageSizeElement.text(this.niceBytes(this.packageSizeElement.data("value")));
    }

    // https://www.codegrepper.com/code-examples/javascript/kb+to+mb+to+gb+jquery
    niceBytes(x) {
        const units = ['bytes', 'KB', 'MB', 'GB', 'TB', 'PB', 'EB', 'ZB', 'YB'];
        let l = 0, n = parseInt(x, 10) || 0;
        while (n >= 1024 && ++l) {
            n = n / 1024;
        }
        return (n.toFixed(n < 10 && l > 0 ? 1 : 0) + ' ' + units[l]);
    }
}
