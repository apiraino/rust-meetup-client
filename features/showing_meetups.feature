Feature: Showing Meetups
  As a member of Rust FVG
  I want to see the future meetups scheduled
  In order to decide if I want to make reservations

  Background:
    Given I'm logged in as a member of Rust FVG
    And the following meetups are registered:
      | 01/01/1999 | Meetup 1 |
      | 01/02/2000 | Meetup 2 |
      | 01/01/2001 | Meetup 3 |

  Scenario: Listing future meetups
    Given today is 01/01/2000
    When I display the list of future meetups
    Then I see the following meetups:
      | 01/02/2000 | Meetup 2 |
      | 01/01/2001 | Meetup 3 |

  Scenario: Listing no future meetup
    Given today is 02/01/2001
    When I display the list of future meetups
    Then I see a no results message

  Scenario: Displaying single future meetup
    #todo