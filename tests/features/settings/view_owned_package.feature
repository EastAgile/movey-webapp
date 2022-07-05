Feature: View owned package

  Scenario: View owned packages
    Given I am a user on Movey
    And I am signed in
    And I upload some packages
    And I visit the My packages page
    Then I should see the My packages page
    And I should see the list of packages that I uploaded
