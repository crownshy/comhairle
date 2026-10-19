# A Polis step has two polls: a preview poll while the conversation is a draft,
# and a fresh live poll created at launch. Ported from #794.

Feature: Preview test data does not leak into the launched conversation
  As an admin staging a conversation
  I want my preview click-testing to stay in the preview polls
  So that the launched conversation starts clean for real participants

  Background:
    Given a draft conversation with steps Learn, Polis 1, Survey, Polis 2

  Scenario: Click-testing in preview writes only to the preview polls
    When the host opens the conversation in preview
    And the host posts a statement in Polis 1
    And the host answers a few questions in the Survey step
    And the host votes agree and disagree in Polis 1 and Polis 2
    Then those votes and survey answers are stored against the preview polls and preview config
    And no live poll exists yet

  Scenario: Launch creates fresh live polls without the preview votes or answers
    Given the host has click-tested Polis 1, the Survey, and Polis 2 in preview
    When the conversation is launched
    Then Polis 1 and Polis 2 each get a fresh live poll
    And the preview votes do not appear in the live polls
    And the preview survey answers do not appear in the launched Survey data

  Scenario: Only seed statement text moves from preview to the live poll
    Given the host wrote 2 seed statements in Polis 1 during preview
    And a participant statement was submitted in the Polis 1 preview poll
    When the conversation is launched
    Then the 2 seed statements appear in the Polis 1 live poll
    And the participant statement from preview does not appear in the live poll

  @bug-796
  Scenario: Rejected seed statements are not carried into the live poll
    Given the host wrote 3 seed statements in Polis 1 during preview
    And the host rejected one of them
    When the conversation is launched
    Then only the 2 seeds that were not rejected appear in the Polis 1 live poll

  @bug-797
  Scenario: After launch the moderation list shows the live poll, not the preview
    Given the host rejected a statement in the Polis 1 preview
    When the conversation is launched
    And the moderator opens moderation for Polis 1
    Then the list shows the live poll's statements
    And none of the preview poll's statements or moderation decisions are listed
