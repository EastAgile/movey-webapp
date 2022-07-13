Feature: Send contact request

  @wio
  Scenario: Guest views
    Given I am a guest / unregistered user
    When I access the Contact us page
    And I select a request category
    And I enter my name
    And I enter my email
    And I enter my request message
    And I click on 'Submit' button
    Then The system should received a Contact Request email

  Scenario: Logged-in user
    Given I am a user on Movey
    And I am signed in
    When I access the Contact us page
    And I select a request category
    And I enter my request message
    Then The system should received a Contact Request email
