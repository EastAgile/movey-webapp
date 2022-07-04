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
        | 'Most Downloads'   | most_downloads    |
        | 'Newly Added'      | newly_added       |

#    Scenario Outline: User access search result page with invalid field
#        Given I access search result page with field param of <invalid_field>
#
#        Examples:
#            | invalid_field    |
#            | 'hmm'            |
#            | 'invalid field'  |
#            | ''  |
#
#    Scenario Outline: User access search result page with invalid page number
#        Given I access search result page with page param of <invalid_page>
#
#        Examples:
#            | invalid_page                                           |
#            | '-1'                                                   |
#            | '0'                                                    |
#            | '1111122223333344445555666667777888899999'             |
#            | '1.1'                                                  |
#            | ''                                                  |
#
#    Scenario Outline: User access search result page with invalid order
#        Given I access search result page with order param of <invalid_order>
#
#        Examples:
#            | invalid_order      |
#            | 'DeSc'             |
#            | 'aSc'              |
#            | 'hmm'              |
#            | ''                 |
