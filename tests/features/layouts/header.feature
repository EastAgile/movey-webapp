Feature: Header

    Scenario: User views search overlay
        Given I am a guest / unregistered user
        Given There are packages in the system
        When I access the package details page
        When I click on the search icon on the dark header
        Then I should see the header search overlay
