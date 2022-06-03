class Profile {
  constructor() {
    this.currentPassword = document.getElementById('current-password')
    this.newPassword = document.getElementById('new-password')
    this.passwordConfirm = document.getElementById('password-confirm')
    this.saveButton = document.getElementById('save-btn')
    this.discardButton = document.getElementById('discard-btn')
    this.init()
  }

  init() {
    let changeSaveButtonState = () => {
      let are_password_fields_valid =  this.currentPassword.value?.length >= 8
          && this.newPassword.value?.length >= 8
          && this.passwordConfirm.value?.length >= 8
          && this.newPassword.value === this.passwordConfirm.value
      this.saveButton.disabled = !are_password_fields_valid
    }
    this.currentPassword.addEventListener('input', changeSaveButtonState)
    this.newPassword.addEventListener('input', changeSaveButtonState)
    this.passwordConfirm.addEventListener('input', changeSaveButtonState)

    this.discardButton.addEventListener('click', () => {
      this.currentPassword.value = ''
      this.newPassword.value = ''
      this.passwordConfirm.value = ''
    })
  }
}
