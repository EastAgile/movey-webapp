class Collaborator {
  constructor() {
    this.add_collaborators = $(".add_collaborators_btn");
    this.collaborators_modal = $("#new_collaborator_modal");

    this.owner_btn = $(".ownership_btn.transfer");
    this.transfer_modal = $("#transfer_owner_modal");

    this.remove_btn = $(".ownership_btn.remove");
    this.remove_modal = $("#remove_owner_modal");

    this.newTokenItemTemplate = $(".token-item-template .token-item");
    this.invite_btn = $(".collaborators_btn.add");
    this.inputEmail = $(".add_collaborators_form #submit-btn");

    this.userName = "";
    this.userNameInput = $(".collaborators_input");
    this.packageName = $(".package-name")[0];

    this.init();
  }

  init() {
    $(document).foundation();
    $(".token-created-at").timeago();

    this.invite_btn.click(() => {
      this.inviteCollaborator();
      this.collaborators_modal.foundation("close");
    });

    this.add_collaborators.click(() => {
      $("#new_collaborator_modal").foundation("open");
    });

    this.collaborators_modal.find(".submit").on("click", () => {
      this.collaborators_modal.foundation("close");
    });

    this.collaborators_modal.find(".cancel").on("click", () => {
      this.collaborators_modal.foundation("close");
    });

    this.owner_btn.each(function(){
      console.log($(this))
      $( this ).click(
        (e) => {
          $('#collaborator_email').text(e.target.parentElement.parentElement.querySelector('.collaborator_name').innerText) 
          $("#transfer_owner_modal").foundation("open")
        }
      )
      
    });
    this.remove_btn.each(function(){
      console.log($(this))
      $( this ).click(
        () => {
          $("#remove_owner_modal").foundation("open")
        }
      )
    });

    this.transfer_modal.find(".submit").on("click", () => {
      console.log("transfer");
      this.transferOwnership()
      this.transfer_modal.foundation("close");
    });
    this.transfer_modal.find(".cancel").on("click", () => {
      console.log("cancel transfer");
      this.transfer_modal.foundation("close");
    });

    this.remove_modal.find(".submit").on("click", () => {
      console.log("remove");
      this.remove_modal.foundation("close");
    });
    this.remove_modal.find(".cancel").on("click", () => {
      console.log("cancel remove");
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
    if (!collaboratorEmail) return;
    let collaboratorUrl =
      "/api/v1/packages/" + this.packageName.innerHTML + "/collaborators";
    console.log(this.userName);
    $.ajax({
      type: "POST",
      dataType: "json",
      url: collaboratorUrl,
      contentType: "application/json",
      processData: false,
      headers: {},
      data: JSON.stringify({ user: this.userName }),
      success: (data, status, xhr) => {
        if (data.ok) {
          console.log("ASDASDASD");
          this.updateRow(this.userName, "pending");
        }
      },
      error: function (xhr, status, errorThrown) {
        $(".tokens-error").text(xhr.responseText);
        return errorThrown;
      },
    });
  };

  transferOwnership = () => {
    
    console.log($('#collaborator_email').text());
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
      success: (data, status, xhr) => {
        if (data.ok) {
          console.log("Transfer success");
        }
      },
      error: function (xhr, status, errorThrown) {
        $(".tokens-error").text(xhr.responseText);
        return errorThrown;
      },
    });
  };

  updateRow = (name, status) => {
    console.log("nbew");
    $(".collaborators_table").append(`
      <div class="collaborator_row">
        <div class="email_address collaborator_name">
          ${name} 
        </div>
        <span class="invitation_status">${status}</span>
        <div class="roles">Collaborator</div>
        <div class="permission collaborators_settings">
          <button id="submit-btn" type="submit" class="ownership_btn transfer">
            Transfer
          </button>
          <button id="submit-btn" type="submit" class="ownership_btn remove">
            Remove
          </button>
        </div>
      </div>
    `);
  };
}
