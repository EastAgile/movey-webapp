Feature: Homepage

  Scenario: Guest user accesses homepage
    Given I am a guest / unregistered user
    When I access the Movey website
    Then I should see the Movey home page

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

  Scenario: User see stats
    Given There are packages in the system
    When I access the Movey website
    Then I should see the correct number of packages and package versions
    When I upload a new package to Movey
    And I access the Movey website
    Then I should see the number of packages and package versions increase by 1
