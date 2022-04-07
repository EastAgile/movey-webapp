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

    Scenario: Wrong email
      When I fill in wrong email and submit the form on the sign in page
      Then I should see the error 'Invalid email or password'

    Scenario: Blank email
      When I fill in blank email and submit the form on the sign in page
      Then I should see the error 'Invalid email or password'

    Scenario: Wrong password
      When I fill in wrong password and submit the form on the sign in page
      Then I should see the error 'Invalid email or password'

    Scenario: Blank password
      When I fill in blank password and submit the form on the sign in page
      Then I should see the error 'Invalid email or password'

  Rule: Signed-in user
      
    @wip
    Scenario: Already signed-in user
      Given I am a user on Movey
      And I am signed in
      When I access the Sign in page
      Then I should be redirected to the Dashboard page
