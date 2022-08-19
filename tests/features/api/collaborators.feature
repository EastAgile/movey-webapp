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
      Then I should see an overlay for inviting a collaborator
      When I invite a user to become a collaborator of the package
      Then She (the collaborator) should receive an invitation email
      When She is signed in
      And She accesses her invitation page
      Then She should see an invitation in her invitation page
@wip
    Scenario: Accept invitation through email works
      When She clicks on the link in the email to accept the invitation
      And She accesses the package detail page
      And She click on the collaborators tab
      Then She should see that she is a collaborator of the package
      When She accesses her invitation page
      Then She should see that the invitation is deleted

    Scenario: Email invitation is expired
      When Collaborator invitation is expired
      And She clicks on the link in the email to accept the invitation
      When She accesses her invitation page
      Then She should see that the invitation is deleted

    Scenario: Accept invitation through website works
      When She clicks on the Accept button to accept the invitation
      Then She should see that the invitation is deleted
      When She accesses the package detail page
      And She click on the collaborators tab
      Then She should see that she is a collaborator of the package

    Scenario: Decline invitation through website works
      When She click on the Decline button to decline the invitation
      Then She should see that the invitation is deleted
      When She accesses the package detail page
      And She click on the collaborators tab
      Then She should see that she is not a collaborator of the package
   
    Scenario: Expired invitation returns message
      When Collaborator invitation is expired
      And She clicks on the Accept button to accept the invitation
      Then She should see that the invitation is deleted
      When She accesses the package detail page
      And She click on the collaborators tab
      Then She should see that she is not a collaborator of the package

  Rule: Invite user outside our system to collaborate

    Background:
      Given I am a user on Movey
      And I am signed in
      And I am an owner of a package
      When I access the package detail page of my package
      And I access the package Settings tab
      And I click on add button
      Then I should see an overlay for inviting a collaborator
      When I invite collaborator with a valid email that is not in our system
      Then She (the outsider) should receive an invitation email
@wip
    Scenario: it works
      When She clicks on the link in the email to sign up
      And She fills in the form and submit
      And She verifies her email
      Then She should be redirected to her profile page
      Then She should see an invitation in her invitation page

    Scenario: Expired invitation
      When She clicks on the link in the email to sign up
      And She fills in the form and submit
      And Collaborator invitation is expired
      And She verifies her email
      Then She should be redirected to her profile page
      And She should see that the invitation is deleted
