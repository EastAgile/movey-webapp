Feature: Package collaborators

  Rule: Invite collaborators

    Background:
      Given I am a user on Movey
      And I am signed in
      And I am an owner of a package
      And There are other users on Movey
      When I access the package detail page of my package
      And I access the package Settings tab
      And I click on add button
      And I invite a user to become a collaborator of the package
      Then I should see a message that the invitation has been sent
      And She (the collaborator) should receive an invitation email
      When She is signed in
      And She access her profile page
      Then She should see an invitation in her profile page
    
    Scenario: Accept invitation through email works
      When She clicks on the link in the email to accept the invitation
      Then She should see that she is a collaborator of the package
      When She access her profile page
      And She should see that the invitation is deleted

    Scenario: Email invitation is expired
      When Collaborator invitation is expired
      And She clicks on the link in the email to accept the invitation
      Then She should see the Invalid or Expired page
      When She access her profile page
      And She should see that the invitation is deleted

    Scenario: Accept invitation through website works
      When She clicks on the Accept button to accept the invitation
      Then She should be redirected to the package details page
      And She should see that she is a collaborator of the package
      When She access her profile page
      And She should see that the invitation is deleted

    Scenario: Decline invitation through website works
      When She click on the Decline button to decline the invitation
      Then She should see that the invitation is deleted
      When She access the package details page
      Then She should see that she is not a collaborator of the package

    Scenario: Expired invitation returns message
      When Collaborator invitation is expired
      And She clicks on the Accept button to accept the invitation
      Then She should see that the invitation is deleted
      And She should receive a message that the invitation is expired
      When She access the package details page
      Then She should see that she is not a collaborator of the package

  Rule: Invite user outside our system to collaborate

    Background:
      Given I am a user on Movey
      And I am signed in
      And I am an owner of a package
      When I access the package details page of my package
      And I access the package Settings tab
      And I invite collaborator with a valid email that is not in our system
      Then She (the outsider) should receive an invitation email

    Scenario: it works
      When She clicks on the link in the email to sign up
      And She fills in the form and submit
      And She verifies her email
      Then She should be redirected to her profile page
      And She should see an invitation in her profile page

    Scenario: Expired invitation
      When Collaborator invitation is expired
      When She clicks on the link in the email to sign up
      And She fills in the form and submit
      And She verifies her email
      Then She should be redirected to her profile page
      And She should see that the invitation is deleted
