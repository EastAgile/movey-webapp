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
        And I confirm the message
        Then I should not see my existing token anymore
        When I revoke the new token
        And I confirm the message
        Then I should not see my new token anymore
