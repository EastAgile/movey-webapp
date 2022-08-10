Feature: Send contact

    Background:
        Given I am a guest / unregistered user
        When I access the Movey website
        And I click on the contact link on the footer
        Then I should see the contact form page

    Scenario: User fill and send contact form
        When I fill in form information and submit the form on contact page
        Then I should receive a thank you email

    Scenario: reCaptcha fails due to invalid input secret
        Given The server has an invalid captcha secret key
        When I fill in form information and submit the form on contact page
        Then I should see an error 'invalid-input-secret' and a message to try again
        And I should not receive a thank you email
