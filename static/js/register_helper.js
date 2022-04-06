function onSignIn(googleUser) {
    // Useful data for your client-side scripts:
    let profile = googleUser.getBasicProfile();
    // window.location.href = `/accounts/google/callback?name=${profile.getName()}`;
}

let btn = document.querySelector('.create_account_btn');
let checkbox = document.querySelector('input.i_agree');
checkbox.addEventListener('input', () => {
    btn.disabled = !checkbox.checked;
})
