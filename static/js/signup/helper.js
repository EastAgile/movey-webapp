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

function onSignIn(googleUser) {
    const isSignOut = getCookie('sign_out');
    if (isSignOut === 'true') {
        gapi.auth2.getAuthInstance().signOut();
        document.cookie = 'sign_out=; Path=/; Expires=Thu, 01 Jan 1970 00:00:01 GMT;'
        return
    }
    let profile = googleUser.getBasicProfile()
    window.location.href = `/accounts/google/callback?name=${profile.getName()}`
}

function getCookie(name) {
    const value = `; ${document.cookie}`;
    const parts = value.split(`; ${name}=`);
    if (parts.length === 2) return parts.pop().split(';').shift();
}
