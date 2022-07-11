Feature: Sign in

  Rule: Non signed-in user

    Background:
      Given I am a user on Movey
      And I am not signed in
      When I access the Movey website
      And I click on the Sign in button on the home page
      Then I should see the sign in page

    Scenario: it works
      When I fill in my email and password and submit the form on the sign in page
      Then I should see that Im logged in
      When I access the Sign in page
      Then I should be on the Dashboard page

    Scenario: Wrong email
      When I fill in wrong email and submit the form on the sign in page
      Then I should see the error 'Invalid email or password! Try again.'

    Scenario: Blank email
      When I fill in blank email and submit the form on the sign in page
      Then I should see the error 'Invalid email or password! Try again.'

    Scenario: Wrong password
      When I fill in wrong password and submit the form on the sign in page
      Then I should see the error 'Invalid email or password! Try again.'

    Scenario: Blank password
      When I fill in blank password and submit the form on the sign in page
      Then I should see the error 'Invalid email or password! Try again.'

  Rule: Signed-in user without remember me option

    Scenario: Default session expired
      Given I am a user on Movey
      And I am signed in
      When I close all browser tabs and reopen my browser
      And I access the Dashboard page
      Then I should see the sign in page

  Rule: Permanently signed-in user

    Background:
      Given I am a user on Movey
      And I am signed in with remember me option

    Scenario: Permanent session works
      When I close all browser tabs and reopen my browser
      And I access the Dashboard page
      Then I should be on the Dashboard page

    Scenario: Permanent session invalid
      When my account is deleted but my browser is not signed out
      And I close all browser tabs and reopen my browser
      And I access the Dashboard page
      Then I am signed out of my account and redirected to Sign in page

    Scenario: Permanent session expired
      When my permanent session is expired
      And I access the Dashboard page
      Then I should see the sign in page

  Rule: Unverified user

    Scenario: Unverified user
      Given I registered an account and have not activated it
      When I sign in into my account
      Then I should see the error 'Your account has not been activated.'
