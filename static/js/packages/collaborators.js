class Collaborator {
  constructor() {
    this.add_collaborator_btn = $(".add_collaborators_btn")
    this.transfer_owner_btn = $(".ownership_btn.transfer")
    this.remove_btn = $(".ownership_btn.remove")
    this.invite_btn = $(".collaborators_btn.add")

    this.collaborators_modal = $("#new_collaborator_modal")
    this.remove_modal = $("#remove_owner_modal")
    this.transfer_modal = $("#transfer_owner_modal")
    this.success_modal = $('#success_modal')

    this.userName = ''
    this.userNameInput = $(".collaborators_input")
    this.packageName = $(".package-name")[0]

    this.init()
  }

  init() {
    $(document).foundation()
    $(".token-created-at").timeago()

    this.invite_btn.click(() => {
      this.inviteCollaborator();
      this.collaborators_modal.foundation("close");
    });

    this.add_collaborator_btn.click(() => {
      $("#new_collaborator_modal").foundation("open");
    });

    this.collaborators_modal.find(".submit").on("click", () => {
      this.collaborators_modal.foundation("close");
    });

    this.collaborators_modal.find(".cancel").on("click", () => {
      this.collaborators_modal.foundation("close");
    });

    this.transfer_owner_btn.click(e => {
      this.current_transfer_target = $(e.target)
      $('#collaborator_email').text(e.target.parentElement.parentElement.querySelector('.email_address').innerText)
      this.transfer_modal.foundation("open")
    });
    this.remove_btn.click(this.removeBtnListener);

    this.transfer_modal.find(".submit").on("click", () => {
      this.transferOwnership()
      this.transfer_modal.foundation("close");
    });
    this.transfer_modal.find(".cancel").on("click", () => {
      this.transfer_modal.foundation("close");
    });

    this.remove_modal.find(".submit").on("click", () => {
      this.removeCollaborator()
      this.remove_modal.foundation("close");
    });
    this.remove_modal.find(".cancel").on("click", () => {
      this.remove_modal.foundation("close");
    });

    this.userNameInput.change(() => {
      this.userName = this.userNameInput.val();
    });

    // handle required input
    $("#user_email").change(() => {
      $(".add_collaborators_btn").css("background-color", "var(--blue-color)");
    });
  }

  inviteCollaborator = () => {
    const collaboratorEmail = $(".collaborators_input").val();
    if (!collaboratorEmail || !this.userName) return;
    let collaboratorUrl =
      "/api/v1/packages/" + this.packageName.innerHTML + "/collaborators/create";
    $.ajax({
      type: "POST",
      dataType: "json",
      url: collaboratorUrl,
      contentType: "application/json",
      processData: false,
      headers: {},
      data: JSON.stringify({ user: this.userName }),
      success: (data) => {
        if (data.ok) this.updateRow(this.userName, "Collaborator")
        else this.updateRow(this.userName, "External");
        $('#success_modal_message').text(data.msg);
        this.success_modal.foundation("open");

      },
      error: (data) => {
        $('#success_modal_message').text(data.responseJSON.msg);
        this.success_modal.foundation("open");
      },
    });
  };

  transferOwnership = () => {
    let collaboratorUrl =
      "/api/v1/packages/" + this.packageName.innerHTML + "/transfer";

    $.ajax({
      type: "POST",
      dataType: "json",
      url: collaboratorUrl,
      contentType: "application/json",
      processData: false,
      headers: {},
      data: JSON.stringify({ user: $('#collaborator_email').text() }),
      success: (data) => {
        $('#success_modal_message').text(data.msg);
        this.success_modal.foundation("open");
        this.current_transfer_target.attr('class', 'hidden-btn')
        this.current_transfer_target.parent().parent()
            .find('.collaborator_name')
            .after(`
                <div class="sending_status">ownership invitation sent</div>
            `)
        this.current_transfer_target = undefined
      },
      error: (data) => {
        $('#success_modal_message').text(data.responseJSON.msg);
        this.success_modal.foundation("open");
      },
    });
  };

  removeCollaborator = () => {
    let collaboratorUrl =
        "/api/v1/packages/" + this.packageName.innerHTML + "/collaborators/remove";

    $.ajax({
      type: "DELETE",
      dataType: "json",
      url: collaboratorUrl,
      contentType: "application/json",
      processData: false,
      headers: {},
      data: JSON.stringify({ user: $('#removed_email').text() }),
      success: (data) => {
        $('#success_modal_message').text(data.msg);
        this.success_modal.foundation("open");
        window.location.reload()
        // TODO: show `transfer` button if it is a PendingOwner, otherwise delete the row
      },
      error: (data) => {
        $('#success_modal_message').text(data.responseJSON.msg);
        this.success_modal.foundation("open");
      },
    });
  };

  updateRow = (name, role) => {
    $(".collaborators_table").append(`
      <div class="collaborator_row">
        <div class="email_address ${role.toLowerCase()}_name">
          ${name}
        </div>
        <div class="sending_status">${role.toLowerCase()} invitation sent</div>
        <div class="roles">
          <p class="collaborator">${role}</p>
        </div>
        <div class="permission collaborators_settings">
          <button type="submit" 
              ${role==='Collaborator' ? '': 'class="hidden-btn"'}>
            Transfer
          </button>
          <button type="submit" class="ownership_btn remove">
            Remove
          </button>
        </div>
    `);
    let remove_btn = $(".ownership_btn.remove").last()
    remove_btn.click(this.removeBtnListener);
  };

  removeBtnListener = e => {
    console.log(e.target.parentElement.parentElement.querySelector('.email_address.collaborator_name'))
    $('#removed_email').text(e.target.parentElement.parentElement.querySelector('.email_address').innerText)
    this.remove_modal.foundation("open")
  }
}
