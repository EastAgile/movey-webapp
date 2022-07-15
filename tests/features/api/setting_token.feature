Feature: Cannot use settings token api

  Rule: Guest user
    Background:
      Given I am a guest / unregistered user
      When I access the Movey website

    Scenario: Guest cannot use api to create token
      When I try to use api to create a token
      Then I should see that I am not authorized to do so

    Scenario: Guest cannot use api to delete token using api
      When I try to use api to delete a token
      Then I should see that I am not authorized to do so

  Rule: Invalid user
    Background:
      Given I am a user on Movey
      And I am signed in
      When My account is deleted but my browser is not signed out
      And I access the Movey website

    Scenario: Invalid user cannot use api to create token
      When I try to use api to create a token
      Then I should see that I am not authorized to do so

    Scenario: Invalid user cannot use api to delete token using api
      When I try to use api to delete a token
      Then I should see that I am not authorized to do so
