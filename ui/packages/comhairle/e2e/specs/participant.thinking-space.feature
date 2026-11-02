# Answering questions with AI follow-ups, then confirming a summary.
# Cases T-01 to T-13 from the participant journey checklist.

Feature: Thinking Space step
  As a participant
  I want to answer questions, explore follow-ups and confirm a summary
  So that organisers hear my reasoning in my own words

  Background:
    Given a live conversation with steps Prioritization, Thinking Space and Lived Experience
    And the Thinking Space step has 2 questions and Minimum follow-ups 1
    And a participant who has joined as a guest and reached the Thinking Space step

  @smoke
  Scenario: T-01 Answer a question
    When the participant opens question 1
    Then Continue is disabled while the answer is empty
    When the participant types an answer and presses Shift+Enter
    Then a new line is added
    When they press Enter
    Then Continue reads Saving while the answer saves
    And the answer is saved

  Scenario: T-02 Follow-up picker
    Given the participant has answered question 1
    Then up to 5 follow-up questions are offered
    When they press Pick for me
    Then one follow-up is chosen for them
    When they type a draft and press Pick a different question
    Then the draft is dropped
    And the follow-up goes back into the list

  Scenario: T-03 Minimum follow-ups
    Given the participant has answered question 1
    Then they see 1 follow-up to go
    When they answer one follow-up
    Then Continue exploring and Move on show
    And on the last question Finish shows instead of Move on

  Scenario: T-04 Minimum set to 0
    Given Minimum follow-ups is 0
    When the participant answers question 1
    Then only Move on shows
    And on the last question only Finish shows

  Scenario: T-05 Progress
    When the participant answers questions and follow-ups
    Then the progress segment moves forward
    And Question X of 2 moves forward

  Scenario: T-06 Follow-ups fail to load
    Given the participant has answered question 1
    When the network goes offline before follow-ups load
    Then they see Couldn't load follow-up questions with Try again

  Scenario: T-07 Reload mid-question
    Given the participant has typed a draft answer
    When they reload the page
    Then the draft is gone
    And they resume at the first unfinished question

  # Likely bug: the save indicator shows Saving even after the save finished.
  @smoke @likely-bug
  Scenario: T-08 Summary
    Given the participant has finished both questions
    Then a recap and a generated summary show
    When they edit the summary and click away
    Then the edit is saved
    And the save indicator says Saved

  Scenario Outline: T-09 Answer more questions <case>
    Given the participant is on the summary
    When they press I want to answer more questions
    And <action>
    And press Finish
    Then <outcome>

    Examples:
      | case                | action                  | outcome                              |
      | with a new answer   | add an answer           | a new summary round is generated     |
      | without new answers | add nothing             | no new summary round is generated    |

  Scenario: T-10 Consent before sharing
    Given Ask before sharing summaries is on
    And the participant is on the summary
    When they press Confirm & Save
    Then a dialog offers Keep private and Share with organisers
    When they choose Share with organisers
    Then their choice is saved
    When they flip the share toggle in the header
    Then the new choice is saved straight away

  # Likely bug: pager Next skips Confirm & Save and consent, so no choice is recorded.
  @likely-bug
  Scenario: T-11 Pager Next on the summary asks for consent
    Given Ask before sharing summaries is on
    And the participant is on the summary
    When they press the pager Next instead of Confirm & Save
    Then the consent dialog shows
    And their choice is recorded before the step completes

  # Likely bug: answers are matched by question text, so rewording orphans them.
  @likely-bug
  Scenario: T-12 Admin rewords a question
    Given the participant has answered question 1
    When the admin edits the wording of question 1
    And the participant reloads the page
    Then their answer is still attached to question 1

  Scenario: T-13 Back during questions
    Given the participant is on question 2
    When they press the pager Back
    Then they leave the step, because there is no going back to question 1 inside the tool
