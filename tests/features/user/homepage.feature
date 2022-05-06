Feature: Homepage

  Scenario: Guest user accesses homepage
    Given I am a guest / unregistered user
    When I access the Movey website
    Then I should see the Movey home page
  @qqq
  Scenario: Guest user search for packages
    Given I am a guest / unregistered user
    Given There are packages in the system
    When I access the Movey website
    And I search for package on the search bar
    Then I should see the dropdown show matching packages
    When I click on an item in the dropdown
    Then I should be redirected to that package detail page
    When I press enter on the search bar or click on search icon
    Then I should be redirected to the search results page