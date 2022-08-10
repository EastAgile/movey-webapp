Feature: Public profile

    Scenario: it works
        Given There are packages in the system
        When I access a public profile
        Then I should see packages that they own

    Scenario: Access profile from package details page
        Given There are packages in the system
        When I access a package details page
        Then I should see owner information of that package
        When I click on the owner name
        Then I should see the public profile of the owner
        And I should see the package on the list of owned packages
