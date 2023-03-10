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
      Then I should see text 'Collaborator invitation is created successfully.'
      When I close the invite modal
      Then I should see the invited collaborator email
      # make sure the result is the same after reloading
      When I access the package Settings tab
      Then I should see the invited collaborator email
      Then She (the collaborator) should receive an invitation email
      When She is signed in
      And She accesses her invitation page
      Then She should see an invitation in her invitation page

    Scenario: Accept invitation through email
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

    Scenario: Accept invitation through website
      When She clicks on the Accept button to accept the invitation
      Then She should see that the invitation is deleted
      When She accesses the package detail page
      And She click on the collaborators tab
      Then She should see that she is a collaborator of the package

    Scenario: Decline invitation through website
      When She click on the Decline button to decline the invitation
      Then She should see that the invitation is deleted
      When She accesses the package detail page
      And She click on the collaborators tab
      Then She should see that she is not a collaborator of the package

    Scenario: Accept an expired invitation
      When Collaborator invitation is expired
      And She clicks on the Accept button to accept the invitation
      Then She should see that the invitation is deleted
      When She accesses the package detail page
      And She click on the collaborators tab
      Then She should see that she is not a collaborator of the package

    Scenario: Send invitation twice
      When She is signed out
      Given I am signed in
      When I access the package detail page of my package
      And I access the package Settings tab
      And I click on add button
      And I invite a user to become a collaborator of the package
      Then I should see text 'Invitation already sent.'

    Scenario: She invites another user
      When She clicks on the Accept button to accept the invitation
      Then She should see that the invitation is deleted
      When She accesses the package detail page
      And She click on the collaborators tab
      Then She should see that she is a collaborator of the package
      When I click on add button
      Then I should see an overlay for inviting a collaborator
      When She invite another user to become a collaborator of the package
      Then I should see text 'Collaborator invitation is created successfully.'

    Scenario: Anonymous cannot see pending collaborators
      When She accesses the package detail page
      And She click on the collaborators tab
      Then I should not see the add button
      When She is signed out
      When I access the package detail page of my package
      And I access the package Settings tab
      Then I should not see the list of pending collaborators
      And I should not see the add button

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
      Then I should see text 'This account is not a Movey user. We are trying to invite this person to join you as a collaborator.'
      When I close the invite modal
      Then I should see the invited external email
      # make sure the result is the same after reloading
      When I access the package Settings tab
      Then I should see the invited external email
      Then She (the outsider) should receive an invitation email

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

    Scenario: Anonymous cannot see external invitations
      When She is signed out
      When I access the package detail page of my package
      And I access the package Settings tab
      Then I should not see the list of external invitations
      And I should not see the add button

    Scenario: Send invitation twice
      When I access the package detail page of my package
      And I access the package Settings tab
      And I click on add button
      And I invite collaborator with a valid email that is not in our system
      Then I should see text 'Invitation already sent.'

  Rule: Invite user not in system fails

    Scenario: it works
      Given I am a user on Movey
      And I am signed in
      And I am an owner of a package
      When I access the package detail page of my package
      And I access the package Settings tab
      And I click on add button
      Then I should see an overlay for inviting a collaborator
      When I invite collaborator with a username that is not in our system
      Then I should see text 'This account is not a Movey user. Inform them by entering their email address.'
