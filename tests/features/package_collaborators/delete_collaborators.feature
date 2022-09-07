Feature: Delete collaborators

  Scenario: Delete pending collaborator invitation
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
    When I access the package Settings tab
    Then I should see the invited collaborator email
    When I click the 'Remove' button
    Then I should see a remove owner modal with text 'You are removing this collaborator "collaborator@host.com" from package "test package"'
    When I click the 'Confirm' button
    Then I should see the invitation is deleted

  Scenario: Delete external invitation
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
      When I click the 'Remove' button
      Then I should see a remove owner modal with text 'You are removing this collaborator "not_in_system@host.com" from package "test package"'
      When I click the 'Confirm' button
      Then I should see the invitation is deleted

  Scenario: Delete ownership transfer invitation
    Given I am a user on Movey
    And I am signed in
    And I am an owner of a package
    And There are other collaborators who work on that package
    When I access the package detail page of my package
    And I access the package Settings tab
    And I transfer ownership to a collaborator
    Then I should see a modal with text 'Ownership transfer invitation is created successfully.'
    When I close the modal
    When I click the 'Remove' button
    Then I should see a remove owner modal with text 'You are removing this collaborator "collaborator@host.com" from package "test package"'
    When I click the 'Confirm' button
    Then I should see the ownership transfer invitation is deleted
