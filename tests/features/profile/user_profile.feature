Feature: View user profile

    Background:
        Given I am a user on Movey
        And I am signed in
        And I visit the root page
        And I visit the Profile page
        Then I should see the Profile page

    @wip
    Scenario: User changes password
        When I enter my current password into 'Current password' textbox
        And I enter new valid password into 'New password' textbox
        And I repeat the same new valid password into 'Repeat new password' textbox
        And I click on 'Save' button
        Then I should see a popup with text 'Change password successfully'
        And I am signed out of my account and redirected to Sign in page
        And I should be able to sign in again with new password

    Scenario: User discard changes
        When I enter some texts into arbitrary textboxes
        And I click on 'Discard' button
        Then I should see all textboxs return to blank