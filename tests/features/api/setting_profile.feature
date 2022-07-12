Feature: Setting profile

  Rule: Signed in user

  Background:
    Given I am a user on Movey
    And I am signed in

    Scenario: it works
      When I use the api to get my profile
      Then I should get information about my profile

    Scenario: Return nothing for deleted user
      When My account is deleted but my browser is not signed out
      And I use the api to get my profile
      Then I should get an empty response from the api

    Scenario: Display sign up button for deleted user
      When My account is deleted but my browser is not signed out
      And I access the Movey website
      Then I should see that my browser is signed out
      And I should see that sign up button is displayed

  Rule: Signed in user with remember me option

  Background:
    Given I am a user on Movey
    And I am signed in with remember me option

  Scenario: Return nothing for deleted user
    When My account is deleted but my browser is not signed out
    And I use the api to get my profile
    Then I should get an empty response from the api
    
  Scenario: Clear remember me cookie for deleted user
    When My account is deleted but my browser is not signed out
    And I access the Movey website
    Then I should see that my browser is signed out
    And I should see that sign up button is displayed

  # Scenario: Request error...
