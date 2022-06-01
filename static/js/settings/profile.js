class Profile {
  constructor() {
    this.saveButton = $(".save")
    this.currentPassword = $(".current-password")
    this.newPassword = $(".new-password")
    this.passwordConfirm = $(".password-confirm")
    this.init()
  }

  init() {
    let changeSaveButtonState = event => {
      this.saveButton.disabled = (
        event.target.validity.typeMismatch
        || this.newPassword.validity.typeMismatch
        || this.passwordConfirm.validity.typeMismatch
      )
    }
    this.currentPassword.addEventListener("change", changeSaveButtonState);
    this.newPassword.addEventListener("change", changeSaveButtonState);
    this.passwordConfirm.addEventListener("change", changeSaveButtonState);

    this.saveButton.click(() => {
      $.ajax({
        type: 'PUT',
        dataType: "json",
        url: '/api/v1/change-password',
        contentType: "application/json",
        processData: false,
        headers: {},
        data: JSON.stringify({
          "password": this.newPassword.val(),
          "repeatPassword": this.passwordConfirm.val()
        }),
        success: function (data, status, xhr) {
          $(".created-token").text(data.token)
          return data
        },
        error: function (xhr, status, errorThrown) {
          $(".error").text(xhr.responseText)
          return errorThrown
        },
      })
    })
  }
}