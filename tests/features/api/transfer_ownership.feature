Feature: Transfer ownership

  Background:
    Given I am a user on Movey
    And I am signed in
    And I am an owner of a package
    And There are other collaborators who work on that package
    When I access the package detail page of my package
    And I access the package Settings tab
    And I transfer ownership to a collaborator
    Then I should see a modal with text 'Ownership transfer invitation is created successfully.'
    When I close the modal
    Then She (the collaborator) should receive an ownership invitation email
    When She is signed in
    And She accesses her invitation page
    Then She should see an ownership invitation in her profile page

  Scenario: it works
    When She clicks on the Accept button to accept the transfer
    Then She should see that the invitation is deleted
    When She accesses the package detail page
    And She click on the collaborators tab
    Then She should see that she is the owner of the package
    Then She should see that I am a collaborator of the package

  Scenario: Reject ownership invitation
    When She clicks on the Decline button to decline the transfer
    Then She should see that the invitation is deleted
    When She accesses the package detail page
    And She click on the collaborators tab
    Then She should see that she is not the owner of the package
    And She should see that I am the owner of the package
    And She should see that she is a collaborator of the package

  Scenario: Expired ownership invitation
    When The transfer ownership invitation is expired
    When She clicks on the Accept button to accept the transfer
    Then She should see that the invitation is deleted
    When She accesses the package detail page
    And She click on the collaborators tab
    Then She should see that she is a collaborator of the package
    And She should see that I am the owner of the package
