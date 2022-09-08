Feature: Public profile

    Scenario: it works
        Given There are packages in the system
        When I access a public profile
        Then I should see packages that he/she owns

    Scenario: Access profile from package details page
        Given There are packages in the system
        When I access a package details page
        Then I should see the owner information
        When I click on the owner name
        Then I should see the public profile
        And I should see packages that he/she owns
