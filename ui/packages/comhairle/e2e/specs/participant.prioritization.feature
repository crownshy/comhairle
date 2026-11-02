# Rating proposals against questions, then reviewing your answers.
# Cases R-01 to R-15 from the participant journey checklist.

Feature: Prioritization step
  As a participant
  I want to rate each proposal and review my answers
  So that my priorities are recorded the way I meant them

  Background:
    Given a live conversation with steps Learn, Prioritization and Thinking Space
    And the Prioritization step has 3 proposals and Minimum proposals to review 2
    And a participant who has joined as a guest and reached the Prioritization step

  @smoke
  Scenario: R-01 First proposal
    When the participant opens the step
    Then Proposal 1 of 3 shows with its questions

  Scenario Outline: R-01 First proposal on a step with <case>
    Given the Prioritization step has <case>
    When the participant opens the step
    Then a clear message explains there is nothing to rate

    Examples:
      | case         |
      | no proposals |
      | no questions |

  @smoke
  Scenario: R-02 Missing answers
    When the participant presses Next with a required question empty
    Then the missing question turns red with a message
    And they stay on the same proposal
    When they answer it
    Then the error clears

  Scenario: R-03 Slider
    When the participant opens a proposal with a slider question
    Then a slider that has not been touched counts as unanswered
    When the participant taps exactly on the slider's resting thumb
    Then the middle value is recorded
    And dragging the slider changes the value
    And the arrow keys change the value

  Scenario: R-04 Submit and advance
    When the participant answers proposal 1 and presses Next
    Then Response submitted shows briefly
    And proposal 2 opens scrolled to the top

  Scenario: R-05 Back to a submitted proposal
    Given the participant has submitted proposal 1
    When they press Back
    Then proposal 1 shows read-only with a Submitted badge
    And the badge tooltip says answers cannot be changed

  Scenario: R-06 Minimum to review
    When the participant submits 1 proposal
    Then they see Reviewed 1 of 2
    When they submit a second proposal
    Then a Skip this step button shows
    And pressing it finishes the step

  @decision-needed
  Scenario: R-07 Skip with a half-filled proposal
    Given the participant has submitted 2 proposals
    And has half-filled proposal 3
    When they press Skip this step
    Then the step finishes
    And the half answers on proposal 3 are dropped

  @smoke
  Scenario: R-08 Review my answers
    Given the participant has submitted all 3 proposals
    Then a thanks card shows
    When they press Review my answers and change one answer
    Then that answer shows an Edited badge
    When they press Next
    Then the change is saved and the step finishes

  # Likely bug: the step completes without saving review edits on this path.
  @likely-bug
  Scenario: R-09 Edit, Back, then Next keeps the edit
    Given the participant is in the review list after submitting all proposals
    When they change an answer
    And press Back to the thanks card
    And press Next
    Then the step finishes
    And reopening the step shows the changed answer

  # Likely bug: it leaves the step instead of returning to the last proposal.
  @likely-bug
  Scenario: R-10 Back on the thanks card
    Given the participant has submitted all 3 proposals
    When they press Back on the thanks card
    Then they are on proposal 3

  @decision-needed
  Scenario: R-11 Random order
    Given Randomise proposal order is on
    When the participant notes the proposal order and reloads the page
    Then the order and the Proposal X of Y numbers stay the same between visits

  Scenario: R-12 Reload with unsubmitted answers
    Given the participant has answered half of proposal 1
    When they reload the page
    Then the typed answers are gone, because nothing saves before Next

  Scenario: R-13 Come back later
    Given the participant has submitted 1 of 3 proposals and left
    When they come back to the step
    Then proposal 2 opens

  Scenario: R-14 Per-section questions
    Given a proposal with two sections and per-section questions
    When the participant answers it
    Then the questions repeat for each section
    And each section's answers are checked before Next

  Scenario: R-15 Optional text answer
    Given a proposal with an optional text question
    Then the question is marked optional
    When the participant leaves it empty
    Then it does not block Next
    When they enter only spaces and submit
    Then no text answer is saved
