class PackageShow {
    constructor() {
        this.readmeElement = $('.package-readme-content');
        this.init();
    }

    init() {
        var converter = new showdown.Converter();
        this.readmeElement.html(converter.makeHtml(this.readmeElement.html()));
    }
}
