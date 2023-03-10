Feature: Sign up

  Rule: Guest user
    Background:
    Given I am a guest / unregistered user
    When I access the Movey website
    And I click on the Sign up button on the home page
    Then I should see the sign up page

    Scenario: it works
      When I fill in my email and password and submit the form on the sign up page
      Then I should see the Verify Your Account page
      And I should receive a verification email
      When I click on the verification link
      Then I should see the Profile page
      
    Scenario Outline: Invalid password
      When I fill in a valid email with value of 'mail@xample.com' and an invalid password with value of <invalid_password>
      Then I should see the error <message>

    Examples:
      | invalid_password           | message                                            |
      | mail@xample.com            | 'Password not strong enough'                       |
      | mailexample                | 'Password not strong enough'                       |
      | 12345                      | 'Password must contain at least 8 characters.'     |

  Rule: Logged-in user

  Scenario: it works
    Given I am a user on Movey
    And I am signed in
    Then I should see the Profile page

  Scenario: Register with existing email
    Given I am a user on Movey
    When I access the Movey website
    And I click on the Sign up button on the home page
    And I fill in my email and password and submit the form on the sign up page
    Then I should receive an email warning that someone is trying to create an account with my email
