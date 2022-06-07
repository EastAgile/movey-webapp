Feature: View search results

    Background:
        Given There are packages in the database
        
    Scenario: User search for package from Homepage
        When I access the Homepage
        And I input a string on search bar
        And I submit the search form
        Then I should see the Search Results page

    Scenario: User sort packages on Search Results page
        Given I have searched for packages with a string
        When I select sort by <option>
        Then I should see the results sorted by <field>

    Examples:
        | option             | field             |
        | 'Name'             | name              |
        | 'Description'      | description       |
        | 'Most Downloads'   | most_downloads    |
        | 'Newly Added'      | newly_added       |
