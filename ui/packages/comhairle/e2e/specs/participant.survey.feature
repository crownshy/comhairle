# HeyForm surveys embedded in a step.
# Cases H-01 to H-12 from the participant journey checklist.

Feature: Survey step
  As a participant
  I want to fill in the survey inside the conversation
  So that I can answer without leaving the page

  Background:
    Given a live conversation with steps Learn, Survey and Prioritization
    And a participant who has joined as a guest and reached the Survey step

  @smoke
  Scenario Outline: H-01 Survey loads in <variant>
    Given the participant is using <variant>
    When they open the Survey step
    Then a skeleton shows, then the form
    And the form is in their language and matches the theme

    Examples:
      | variant    |
      | light mode |
      | dark mode  |
      | Welsh      |

  @smoke
  Scenario: H-02 Submit and move on
    When the participant answers every question
    Then the progress segment fills a little more for each question
    When they submit
    Then about 2 seconds later they move on to the next step

  Scenario Outline: H-03 Pager before submitting a <setting> survey
    Given the Survey step is <setting>
    When the participant opens it without submitting
    Then <pager>

    Examples:
      | setting  | pager                                                    |
      | required | Next is disabled, with no reason given                   |
      | optional | Skip is shown and moves on to the next step              |

  Scenario: H-04 Theme change mid-survey
    Given the participant has answered two questions
    When they toggle the theme from the step menu
    Then the form repaints in the new theme
    And their answers are kept

  Scenario: H-05 Long survey on a phone
    Given a phone viewport
    And a survey with many questions
    When the participant goes through it
    Then the frame grows with each question
    And there are no double scrollbars
    And each question scrolls into view
    And no buttons are covered

  # Likely bug: the 2-second timer is not cancelled, so it may complete the wrong step.
  @likely-bug
  Scenario: H-06 Leave right after submitting
    Given the Learn step is revisitable and completed
    When the participant submits the survey
    And within 2 seconds opens Learn from the step menu
    Then the Survey step is marked done
    And the Learn step is not completed a second time

  @decision-needed
  Scenario: H-07 Revisit a survey
    Given the Survey step is revisitable
    And the participant has submitted it
    When they open it again
    Then a fresh form shows
    When they submit it again
    Then a second submission is recorded

  # Likely bug: some survey parameters are sent with a broken amp; prefix.
  @likely-bug
  Scenario: H-08 Form address
    When the participant opens the Survey step
    Then no parameter in the form request starts with amp;

  Scenario: H-09 Language HeyForm does not support
    When the participant switches to Pashto
    And opens the Survey step
    Then the form shows in English

  Scenario Outline: H-10 Data protocol text when the field is <state>
    Given the Survey step's Data protocol field is <state>
    When the participant opens Find out more, Privacy, on the Survey step
    Then it shows <text>

    Examples:
      | state  | text                         |
      | empty  | the default survey text      |
      | filled | the text the admin entered   |

  Scenario: H-11 Answers reach admin
    When the participant submits the survey
    Then the submission shows in the admin responses
    And it is linked to their participant id

  # Likely bug: there is no error or timeout state, so the skeleton probably shows forever.
  @likely-bug
  Scenario: H-12 Survey host down
    Given the form host is unreachable
    When the participant opens the Survey step
    Then an error message shows instead of a skeleton that never ends
