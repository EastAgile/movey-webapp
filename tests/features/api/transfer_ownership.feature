Feature: Transfer ownership

  Background:
    Given I am a user on Movey
    And I am signed in
    And I am an owner of a package
    And There are other collaborators who work on that package
    When I access the package details page of my package
    And I access the package Settings tab
    And I transfer ownership to a collaborator
    Then She (the collaborator) should receive an ownership invitation email
    When She is signed in
    And She access her profile page
    Then She should see an ownership invitation in her profile page

  Scenario: it works
    When She clicks on the button to accept the transfer
    Then She should be redirected to the package details page
    And She should see that she is the owner of the package
    And She should see that I am a collaborator of the package
  
  Scenario: Expired ownership invitation
    When The transfer ownership invitation is expired
    And She clicks on the button to accept the transfer
    Then She should receive a message that the invitation is expired
    And She should see that the invitation is deleted
    When She access the package details page
    Then She should see that she is a collaborator of the package
    And She should see that I am the owner of the package
