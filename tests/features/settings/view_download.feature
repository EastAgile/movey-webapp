Feature: View download

  Scenario: View download
    Given I am a user on Movey
    And I am signed in
    And I visit the Profile page
    When I click on the downloads tab
    Then I should see the profile downloads page
