Feature: Log out

  Background:
    Given I am a user on Movey

  Scenario: Sign-in then logout
    Given I am signed in
    When I access the Dashboard page
    And I click on the Log out button
    Then I should see the home page

  Scenario: Sign-in with remember me then logout
    Given I am signed in with remember me option
    When I access the Dashboard page
    And I click on the Log out button
    Then I should see the home page
