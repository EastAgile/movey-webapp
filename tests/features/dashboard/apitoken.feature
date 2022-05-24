Feature: API token

  Scenario: able to access API Tokens page
    Given I am an user on Movey
    And I am signed in
#    And I linked my account with Github
    When I access the API Tokens page
    Then I should see the API Tokens page

  Rule: create new API token

    Background:
      Given I am an user on Movey
      And I am signed in
#      And I linked my account with Github
      When I access the API Tokens page
      When I click on the New Token button
      Then I should see the New Token Name text box

    @wip
    Scenario: able to create new API token
      When I enter the new token name
      And I click on the Create button
      Then I should see the new token

    Scenario: failed to create new API token due to duplicate token name
      When I enter a token name that is already existed
      And I click on the Create button
      Then I should see the error 'That name has already been taken.'

    Scenario: failed to create new API token due to maximum number of allowed tokens
      Given I have created the maximum number of allowed tokens
      When I enter new token name
      And I click on the Create button
      Then I should see the error 'Too many tokens created.'
