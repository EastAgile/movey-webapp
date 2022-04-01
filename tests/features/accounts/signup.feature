Feature: Sign up

  Background:
    Given I am a guest / unregistered user
    When I access the Movey website
    And I click on the Sign up button on the home page
    Then I should see the sign up page

    Scenario: it works
      When I fill in my email and password and submit the form on the sign up page
      Then I should see that my account has been created
      
    @wip
    Scenario Outline: Invalid password
      When I fill in a valid email with value of 'mail@xample.com' and an invalid password with value of <invalid_password>
      Then I should see the error <message>

    Examples:
      | invalid_password           | message                          |
      | mail@xample.com            | 'Password not strong enough'     |
      | mailexample                | 'Password not strong enough'     |
