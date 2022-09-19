class Tokens {
  constructor() {
    this.createTokenButton = $(".create-new-token-btn, .create-new-token-mobile-btn")
    this.modal = $('#new_token_modal')
    this.newTokenInput = this.modal.find('input')
    this.newTokenItemTemplate = $('.token-item-template .token-item')
    this.tokensList = $('.tokens-list')
    this.revokeModal = $('#revoke_token_modal')
    this.init()
  }

  init() {
    $(document).foundation()
    $('.token-created-at').timeago();

    this.createTokenButton.click(() => {
      this.modal.foundation('open')
      this.newTokenInput.focus()
    })

    this.modal.find('.new-token-submit-btn').on('click', () => {
      this.modal.foundation('close')
      this.submitNewToken()
      this.newTokenInput.val('')
    })

    this.newTokenInput.on('keypress', (e) => {
      if (e.key == "Enter") {
        this.modal.foundation('close')
        this.submitNewToken()
        this.newTokenInput.val('')
      }
    })

    $('body').on('click', '.copy-token-icon-btn, .copy-token-btn', (e) => {
      const tokenTextElement = $(e.currentTarget).closest('.token-item').find('.token-plaintext')
      navigator.clipboard.writeText(tokenTextElement.text())
      $(e.currentTarget).removeClass('blink')
      $(e.currentTarget).addClass('blink')
    })

    $('body').on('click', '.revoke-token-btn', (e) => {
      const tokenId = $(e.currentTarget).closest('.token-item').data('id')
      this.revokeModal.foundation('open')
      this.revokeModal.find('.revoke-token-confirm').on('click', () => {
        this.revokeToken(tokenId)
      })
    })
    
    this.revokeModal.find('.revoke-token-cancel').on('click', () => {
      this.revokeModal.foundation('close')
    })
  }

  submitNewToken() {
    const tokenName = this.newTokenInput.val();
    if (!tokenName) return
    $.ajax({
      type: 'PUT',
      dataType: "json",
      url: '/api/v1/settings/tokens',
      contentType: "application/json",
      processData: false,
      headers: {},
      data: JSON.stringify({"name": tokenName}),
      success: (data, status, xhr) => {
        $('.no-tokens').remove()
        const newTokenItem = this.newTokenItemTemplate.clone()
        newTokenItem.data('id', data.id)
        newTokenItem.find('.token-name').text(data.name)
        newTokenItem.find('.token-plaintext').text(data.token)

        this.tokensList.append(newTokenItem)
        return data
      },
      error: function (xhr, status, errorThrown) {
        $(".tokens-error").text(xhr.responseText)
        return errorThrown
      },
    })
  }

  revokeToken(id) {
    $.ajax({
      type: 'DELETE',
      dataType: "json",
      url: '/api/v1/settings/tokens/' + id,
      contentType: "application/json",
      processData: false,
      headers: {},
      data: {},
      complete: () => {
        window.location.reload()
      }
    })
  }
}
