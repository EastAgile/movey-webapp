Feature: View user profile

    Background:
        Given I am a user on Movey
        And I am signed in
        And I visit the Profile page
        Then I should see the Profile page

    Scenario: User changes password successfully
        When I enter my current password into 'Current password' textbox
        And I enter new valid password into 'New password' textbox
        And I repeat the same new valid password into 'Repeat new password' textbox
        And I click on 'Save' button
        Then I am signed out of my account and redirected to Sign in page
        And I should see a message with text 'Change password successfully'
        And I should be able to sign in again with new password

    Scenario: User changes password failed because password confirmation mismatches
        When I enter my current password into 'Current password' textbox
        And I enter new valid password into 'New password' textbox
        And I enter different password into 'Repeat new password' textbox
        Then I should see the 'Save' button is disabled

    Scenario: User changes password failed because new passwords is not long enough
        When I enter my current password into 'Current password' textbox
        And I enter an short password into 'New password' textbox
        And I repeat the same short password into 'Repeat new password' textbox
        Then I should see the 'Save' button is disabled

    Scenario: User changes password failed because of wrong current password
        When I enter wrong password into 'Current password' textbox
        And I enter new valid password into 'New password' textbox
        And I repeat the same new valid password into 'Repeat new password' textbox
        And I click on 'Save' button
        Then I should see the error 'Wrong password. Try again!'

    Scenario: User discard changes
        When I enter random texts into whatever textboxes
        And I click on 'Discard' button
        Then I should see all textboxs return to blank

    Scenario Outline: User submits invalid form
        When I enter <invalid_new_password> into 'New password'
        When I enter <invalid_password_confirm> into 'Repeat my password'
        And I click on 'Save' button
        Then I should see the error '<error>'

    Examples:
        | invalid_new_password       | invalid_password_confirm      | error                        |
        | invalid1                    | invalid1                     | Password not strong enough.  |
