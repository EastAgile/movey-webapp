Feature: View Package

    Scenario: User views package page
        Given I am a guest / unregistered user
        Given there is a package in the system
        When I access the package details page
        Then I should see latest information of that package
        When I click on versions of that package
        Then I should see the versions of that package by latest
        When I sort the package versions by oldest
        Then I should see the versions of that package by oldest
        When I click on an older version of the package
        Then I should see the older version of the package
