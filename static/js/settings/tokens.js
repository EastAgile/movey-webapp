class Tokens {
  constructor() {
    this.createTokenButton = $(".create-button")
    this.tokenName = $(".new-token")
    this.init()
  }

  init() {
    this.createTokenButton.click(() => {
      const tokenName = this.tokenName.val();
      if (!tokenName) return
      $.ajax({
        type: 'PUT',
        dataType: "json",
        url: '/api/v1/tokens',
        contentType: "application/json",
        processData: false,
        headers: {},
        data: JSON.stringify({"name": tokenName}),
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
