Feature: Send contact

    Background:
        Given I am a guest / unregistered user
        When I access the Movey website
        And I click on the contact link on the footer
        Then I should see the contact form page

    Scenario: User fill and send contact form
        When I fill in form information and submit the form on contact page
        Then I should receive a thank you email

