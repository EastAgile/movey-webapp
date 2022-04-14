Feature: Log out

Background:
  Given I am a user on Movey

Scenario: Default sign-in then logout
  Given I am signed in
  When I access the Dashboard page
  And I click on the Log out button
  Then I should see the home page

Scenario: Default sign-in then logout
  Given I am signed in with option to keep me signed in
  When I access the Dashboard page
  And I click on the Log out button
  Then I should see the home page
