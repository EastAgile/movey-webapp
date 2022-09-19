Feature: View Package

    Scenario: User views package page
        Given I am a guest / unregistered user
        Given There are packages in the system
        When I access the package details page
        Then I should see latest information of that package
        When I click on versions of that package
        Then I should see the versions of that package by latest
        When I sort the package versions by oldest
        Then I should see the versions of that package by oldest
        When I click on an older version of the package
        Then I should see the older version of the package

    Scenario: User views package that was upload by move-cli
        Given I am a guest / unregistered user
        Given There are packages in the system
        When I access the package details page
        Then I should see the owner information

    Scenario: User views package that was crawled
        Given I am a guest / unregistered user
        Given There are packages in the system
        When I access the package details page of a package that was crawled
        Then I should see a default owner name
        And I should see a banner saying that this package is crawled
        And I should see that banner tell me to create an account
        And I should not see the Collaborators tab

    Scenario: Signed in user views package that was crawled
        Given There are packages in the system
        Given I am signed in
        When I access the package details page of a package that was crawled
        Then I should see a banner saying that this package is crawled
        And I should see that banner tell me to contact for ownership request
        And I should not see the Collaborators tab

    Scenario: User views package that is in a subdir
        Given I am a guest / unregistered user
        And There is a package that is in a subdir
        When I access the package details page of a package that is in a subdir
        Then I should see correct install instruction for that package
        When I click on versions of that package
        And I sort the package versions by oldest
        And I click on an older version of the package
        Then I should see correct install instruction for older version of that package
