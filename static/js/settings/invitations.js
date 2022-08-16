class Invitations {
    constructor() {
      this.acceptBtn = $('.accept');
      this.cancelBtn = $('.cancel');
      this.init();
    }
  
    init() {

      this.acceptBtn.click((event) => {
        let packageId = $(event.target).parent().data("package-id");
        this.submitInvitationResponse(packageId, true);
      });
      this.cancelBtn.click((event) => {
        let packageId = $(event.target).parent().data("package-id");
        this.submitInvitationResponse(packageId, false);
      });
    }
  
    submitInvitationResponse(packageId, accepted) {
      $.ajax({
        type: 'POST',
        dataType: "json",
        url: '/api/v1/owner_invitations',
        contentType: "application/json",
        processData: false,
        headers: {},
        data: JSON.stringify({"package_id": packageId, "accepted": accepted}),
        success: (data, status, xhr) => {
          // TODO:
        },
        error: function (xhr, status, errorThrown) {
          // TODO
        },
      })
    }
  }
  