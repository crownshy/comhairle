# The public conversation report.
# Cases RP-1 and RP-2 from the participant journey checklist.

Feature: Public report
  As a member of the public
  I want to read what came out of the conversation
  So that I can see how views were gathered and what they said

  Background:
    Given a conversation with a published report and two Polis steps
    And a visitor who is not signed in

  Scenario Outline: RP-1 Read the report on <device>
    Given a <device> viewport
    When the visitor opens /report
    Then the title and date show
    And the sections nav highlights the section they are reading as they scroll
    And Back to top shows after they scroll down

    Examples:
      | device  |
      | desktop |
      | phone   |

  # Likely bug: it loads a hard-coded Polis report for every step.
  @likely-bug
  Scenario: RP-2 Polis iframe toggle
    Given Show Polis iframe is on
    When the visitor opens /report
    Then each Polis step's iframe loads that step's own poll report
