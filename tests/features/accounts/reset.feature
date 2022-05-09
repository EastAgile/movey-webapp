Feature: Reset password

  Background:
    Given I have successfully requested a password reset link
      And I have received the email that contains password reset link
      When I access the reset password link
      Then I should see the Reset Password page

    Scenario: Valid password
      When I fill in a valid password and repeat the password correctly
      And I submit the form on Reset Password page
      Then I should see the Password Changed page
      And I should receive an email that confirms password has changed
    
    Scenario: Password mismatch
      When I fill in a valid password and repeat the password incorrectly
      And I submit the form on Reset Password page
      Then I should see the error 'Passwords must match.'

    Scenario: Invalid password
      When I fill in an invalid password <invalid_password> and repeat the password correctly
      And I submit the form on Reset Password page
      Then I should see the error <error_msg>

    Examples:
      | invalid_password    | error_msg                                               |
      | aaaaaaaaaaaaaa      | 'Repeats like "aaa" are easy to guess.'                 |
      | password123         | 'This is similar to a commonly used password.'          |
      | abcd123             | 'Password must contain at least 8 characters.'          |
      | qwertyuiop          | 'Straight rows of keys are easy to guess.'              |

      

    

