class Helper {
    constructor() {
        this.create_account_btn = $('.create_account_btn');
        this.checkbox = $('input#i_agree');
        this.init();
    }

    init() {
        this.checkbox.change(() => {
            this.create_account_btn.prop('disabled', !this.checkbox.prop('checked'));
        })
    }
}
