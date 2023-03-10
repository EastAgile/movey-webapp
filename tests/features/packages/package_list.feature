Feature: View package list

    Background:
        Given There are packages in the database

    Scenario: User see sorted packages
        Given I am on the package list page
        When I select sort by <option>
        Then I should see the packages sorted by <field>

    Examples:
        | option             | field             |
        | 'Name'             | name              |
        | 'Most Stars'       | most_stars        |
        | 'Most Forks'       | most_forks        |
        | 'Newly Added'      | newly_added       |
        | 'Recently Updated' | recently_updated  |
