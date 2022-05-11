Feature: Request password reset link

  Background:
    Given I am not logged in
    When I access the Sign In page
    And I click on the Forgot Passwork link on sign in form
    Then I should see the Forgot Password page
    @wip
    Scenario: Registered email
      Given I have registered an email
      When I fill in a registered email and submit the form on Forgot Password page
      Then I should see the Confirm Request page
      And I should receive an email that contains valid password reset link
    
    Scenario: Unregistered email
      When I fill in an unregistered email and submit the form on Forgot Password page
      Then I should see the Confirm Request page
      But I should not receive an email
