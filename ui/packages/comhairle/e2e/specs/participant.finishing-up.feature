# Thank-you page, feedback and the participant's account pages.
# Cases F-01 to F-09 from the participant journey checklist.

Feature: Finishing up
  As a participant
  I want a clear end to the conversation and control over my account
  So that I know my part is done and how I will hear back

  Background:
    Given a live conversation with steps Learn and Polis
    And Learn is revisitable

  @smoke
  Scenario Outline: F-01 Thank-you page <case>
    Given the conversation has <message>
    When a participant finishes the last step
    Then the thank-you page shows <heading>
    And the revisitable steps are listed

    Examples:
      | case                     | message                 | heading                 |
      | with a thank-you message | a thank-you message set | that message            |
      | without a message        | no thank-you message    | the default heading     |

  Scenario Outline: F-02 Thank-you instructions for <account>
    Given Show thank you page anonymous instructions is on
    When a participant who joined as <account> finishes the last step
    Then they see <instructions>

    Examples:
      | account          | instructions                       |
      | a guest          | their guest code                   |
      | an email account | the text saying we will email them |

  Scenario Outline: F-03 Signup prompts for <account>
    Given Enable signup prompts is on
    When a participant who joined as <account> finishes the last step
    Then they see <prompts>

    Examples:
      | account          | prompts                              |
      | a guest          | upgrade and email sign-up prompts    |
      | an email account | notification switches                |

  # Likely bug: empty feedback can be sent and the text is not cleared afterwards.
  @likely-bug
  Scenario: F-04 Feedback
    Given a participant on the thank-you page
    When they open Feedback and submit some text
    Then a thanks toast shows and the dialog closes
    When they open Feedback again
    Then the text field is empty
    And Submit is disabled until they type something

  # Likely bug: the thank-you text is hard-coded English.
  @likely-bug
  Scenario: F-05 Thank-you page in Welsh
    Given a participant on the thank-you page
    When they switch to Welsh
    Then no English text is left on the page

  Scenario Outline: F-06 Back in after finishing with <revisitable>
    Given <revisitable>
    And a participant who has finished every step
    When they open the conversation page and press Jump back in
    Then <destination>

    Examples:
      | revisitable                | destination                                  |
      | some revisitable steps     | they see a list of the revisitable steps     |
      | no revisitable steps       | they land on the thank-you page              |

  Scenario Outline: F-07 Settings for <account>
    Given a participant who joined as <account>
    When they open Settings
    Then they see <content>

    Examples:
      | account          | content                                                          |
      | a guest          | their guest code and an upgrade option                           |
      | an email account | details and demographics forms, and notification switches        |

  # Likely bug: Load More does nothing and the unread count is unreadable in dark mode.
  @likely-bug
  Scenario: F-08 Notifications
    Given a participant with more notifications than fit on one page
    When they open Notifications and mark one as read
    Then a toast confirms it
    When they press Mark All Read
    Then a toast confirms it and every notification is read
    When they switch to unread only
    Then only unread notifications are listed
    When they press Load More
    Then more notifications load
    And in dark mode the unread count is readable

  Scenario Outline: F-09 Rights page: <page>
    When a participant opens the <page> page
    Then the nav highlights <page>
    And its links work

    Examples:
      | page          |
      | Terms         |
      | Privacy       |
      | Cookies       |
      | Accessibility |

  Scenario: F-09 Cookie preferences
    When a participant opens the Cookies page and presses Cookie preferences
    Then the cookie panel opens
