# Voting on statements and adding your own, from the participant's side.
# Cases P-01 to P-22 from the participant journey checklist. Moderation and the
# preview-to-live handover are in polis.moderation.feature and
# polis.preview-to-live.feature; cases that overlap them point there instead.

Feature: Polis step
  As a participant
  I want to vote on other people's statements and add my own
  So that my views shape where the conversation agrees and splits

  Background:
    Given a live conversation with a Polis step
    And the Polis step has Required votes 5 and 6 seed statements
    And a participant who has joined as a guest

  @smoke
  Scenario: P-01 Vote agree, disagree and pass
    When the participant votes Agree, Disagree and Pass/Unsure on three statements
    Then the next statement loads after each vote
    And the report records the Agree as agree and the Disagree as disagree

  Scenario: P-02 Statement labels
    Given a statement written by another participant has been accepted
    When the participant sees a seed statement
    Then it is labelled Conversation starter
    When they see the participant statement
    Then it is labelled A member of the public commented

  @smoke
  Scenario: P-03 Vote threshold
    When the participant votes 4 times
    Then Next is locked with a reason
    And the progress segment fills by a fifth per vote
    When they vote once more
    Then Next unlocks

  @smoke
  Scenario: P-04 Keep voting or I'm done
    When the participant reaches exactly 5 votes
    Then a prompt says they have voted on 5 statements
    And it offers Keep voting, with how many statements are left, and I'm done voting

  Scenario: P-05 Keep voting
    Given the participant has reached the Keep voting prompt
    When they press Keep voting and vote 5 more times
    Then the prompt comes back

  Scenario: P-06 I'm done voting
    Given the participant has reached the Keep voting prompt
    When they press I'm done voting
    Then the step completes and they move on to the next step

  Scenario: P-07 Voted on everything
    When the participant votes until no statements are left
    Then a screen says they have voted on every opinion in this step so far
    And Next is enabled
    And Add your own opinion still works

  @smoke
  Scenario: P-08 Add your own opinion
    When the participant opens Add your own opinion
    Then Submit is disabled while the field is empty
    When they type 200 characters and try to type more
    Then the counter turns red at 0 and the extra text is cut off
    When they press Submit
    Then a thank-you banner shows
    And after 2 seconds they are back to voting

  Scenario: P-09 Submit and add another
    Given the participant has typed a statement in Add your own opinion
    When they press Submit and add another one
    Then the thank-you banner shows
    And they stay on the form with the field cleared

  Scenario: P-10 Strict moderation hides statements until accepted
    Given No comments shown without moderator approval is on
    And participant A has submitted a statement
    When participant B votes through every statement
    Then B does not see A's statement
    When the moderator accepts A's statement
    And B votes again
    Then B sees A's statement

  # The rejected half of this case is "Rejecting a statement hides it from participants"
  # in polis.moderation.feature.
  Scenario: P-11 Strict moderation off shows pending statements
    Given No comments shown without moderator approval is off
    When participant A submits a statement
    Then participant B sees it while it is still pending

  Scenario: P-12 Reload mid-voting
    Given the participant has voted 3 times
    When they reload the page and vote 2 more times
    Then their count carries on from 3
    And Next unlocks at 5

  Scenario: P-13 Second device
    Given the participant has voted past the threshold on desktop
    When they open the same step on their phone
    Then the threshold is already met

  Scenario: P-14 Translated statements
    Given the conversation languages include Welsh
    And the moderation sync has run
    When the participant switches to Welsh and votes
    Then statements show translated
    And a Translated from toggle reveals the original

  # Likely bug: it goes straight to the voted-on-everything screen and unlocks Next with zero votes.
  @likely-bug
  Scenario Outline: P-15 Empty poll does not unlock Next: <case>
    Given <setup>
    When the participant opens the Polis step
    Then Next stays locked until the vote threshold is met

    Examples:
      | case               | setup                                                              |
      | no statements      | a Polis step with no statements                                    |
      | nothing accepted   | strict moderation is on and no statement has been accepted         |

  # Likely bug: the error card has no buttons and no retry, so they are stuck until they reload.
  @likely-bug
  Scenario: P-16 Statement submit fails
    Given the participant is offline
    When they submit a statement
    Then an error shows with a way to retry
    When they press Back to voting
    Then they can carry on voting

  # Likely bug: the vote counts locally before the server answers and there is no retry.
  @likely-bug
  Scenario: P-17 Vote fails
    Given the participant is offline
    When they vote
    Then an error says the vote was not saved, with a way to retry
    And the vote does not count toward the threshold

  # Likely bug: the prompt only shows when the count equals the threshold exactly.
  @likely-bug
  Scenario: P-18 Admin lowers Required votes
    Given the participant has voted 4 times
    When the admin changes Required votes to 3
    And the participant reloads the page
    Then Next is enabled
    And the Keep voting prompt shows

  # Likely bug: the count comes from the page load and can be wrong after a reload.
  @likely-bug
  Scenario: P-19 Vote count on the prompt
    Given the participant has reached the prompt and pressed Keep voting
    When they reload the page and vote until the prompt shows again
    Then You've voted on N matches the votes they have actually cast

  Scenario: P-20 Opinions left hint
    When the participant reaches the Keep voting prompt
    Then the N more opinions to vote on hint matches the statements actually left

  # P-21 Preview votes after launch: covered by "Launch creates fresh live polls without the
  # preview votes or answers" and "Only seed statement text moves from preview to the live poll"
  # in polis.preview-to-live.feature. The checklist read of the code suspects every preview
  # statement, participant ones included, is re-posted as a seed at launch. If that is
  # confirmed, tag that scenario with the new bug issue rather than adding one here.

  # Known follow-up: the toggle does nothing and should be removed. Delete this scenario then.
  Scenario: P-22 Show remaining statements has no effect on participants
    Given the participant is voting on the Polis step
    When the admin toggles Show remaining statements in Polis Setup
    And the participant reloads the page
    Then what the participant sees is unchanged
