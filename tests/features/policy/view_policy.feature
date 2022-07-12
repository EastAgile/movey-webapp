Feature: Policy

  Rule: Term of use page
    Scenario: Guest views the Terms of use page
      Given I am a guest / unregistered user
      When I access the Terms of use page
      Then I should see the Terms of use page

    Scenario: Signed-in user views the Terms of use page
      Given I am a user on Movey
      And I am signed in
      When I access the Terms of use page
      Then I should see the Terms of use page

  Rule: Policy page
    Scenario: Guest views the Policy page
      Given I am a guest / unregistered user
      When I access the Policy page
      Then I should see the Policy page

    Scenario: Signed-in user views the Policy page
      Given I am a user on Movey
      And I am signed in
      When I access the Policy page
      Then I should see the Policy page

  Rule: Contact us page
    @wip
    Scenario: Guest views the Contact us page
      Given I am a guest / unregistered user
      When I access the Contact us page
      Then I should see the Contact us page

    Scenario: Guest views the Contact us page
      Given I am a user on Movey
      And I am signed in
      When I access the Contact us page
      Then I should see the Contact us page
