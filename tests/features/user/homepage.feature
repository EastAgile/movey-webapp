Feature: Homepage

  Scenario: Guest user accesses homepage
    Given I am a guest / unregistered user
    When I access the Movey website
    Then I should see the Movey home page
