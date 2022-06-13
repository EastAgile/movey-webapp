Feature: View user api tokens
    Scenario: User views token page
        Given I am a user on Movey
        And I am signed in
        And I visit the Profile page
        When I click on the tokens tab
        Then I should see the profile tokens page

    Scenario: User views existing tokens, generate and revoke tokens
        Given I am a user on Movey
        And I have an existing api token
        And I am signed in
        And I visit the profile tokens page
        Then I should see my existing api token
        When I click on create a new token
        And I fill in the token name and submit
        Then I should see a new token created
        When I revoke the existing token
        Then I should not see my existing token anymore
        When I revoke the new token
        Then I should not see my new token anymore

    Scenario: Add token name that already exists
        Given I am a user on Movey
        And I have an existing api token
        And I am signed in
        And I visit the profile tokens page
        When I click on create a new token
        And I fill in the existing token name and submit
        Then I should see the token error that name has already been taken

    Scenario: Add token when max is reached
        Given I am a user on Movey
        And I have created the maximum number of allowed tokens
        And I am signed in
        And I visit the profile tokens page
        When I click on create a new token
        And I fill in the token name and submit
        Then I should see the token error that maximum token is reached
