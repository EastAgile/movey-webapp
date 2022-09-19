class Invitations {
  constructor() {
    this.acceptBtn = $('.accept');
    this.cancelBtn = $('.cancel');
    this.init();
  }

  init() {

    this.acceptBtn.click((event) => {
      let packageId = $(event.target).parent().data("package-id");
      this.submitInvitationResponse(packageId, true, $(event.target));
    });
    this.cancelBtn.click((event) => {
      let packageId = $(event.target).parent().data("package-id");
      this.submitInvitationResponse(packageId, false, $(event.target));
    });
  }

  submitInvitationResponse(packageId, accepted, targetElement) {
    $.ajax({
      type: 'POST',
      dataType: "json",
      url: '/api/v1/collaborators/handle',
      contentType: "application/json",
      processData: false,
      headers: {},
      data: JSON.stringify({ "package_id": packageId, "accepted": accepted }),
      success: () => {
        // reload to update database because ajax response need time to load new change
        targetElement.parent().parent().remove()
        console.log("OK");
      },
      error: function () {
        window.location.reload();
      },
    })
  }
}
