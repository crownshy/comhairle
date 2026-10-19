# Moderation targets the preview poll while the conversation is a draft and the
# live poll after launch. Ported from #795.

Feature: Polis statement moderation
  As a moderator
  I want to review and reject Polis statements
  So that unsuitable statements never reach participants

  Background:
    Given a conversation with Polis steps Polis 1 and Polis 2

  Scenario: Moderation offers status filters
    When the moderator opens moderation for Polis 1
    Then it offers the filters All, Seeded, Accepted, Pending and Rejected

  Scenario: Rejecting a statement hides it from participants
    Given the conversation is live
    And a pending statement exists in the Polis 1 live poll
    When the moderator rejects it
    Then participants no longer see the statement
    And it is listed under the Rejected filter

  Scenario: Statements submitted in preview show up for moderation
    Given the conversation is a draft
    And a statement was submitted in the Polis 1 preview poll
    When the moderator opens moderation for Polis 1 and syncs statements
    Then the statement is listed as pending
    And accepting it makes it show in Insights

  Scenario: A statement the host adds through Add your own opinion is not a seed
    Given the conversation is a draft
    When the host submits a statement through Add your own opinion in the Polis 1 preview
    And the moderator syncs statements
    Then the statement is not marked as a seed

  Scenario: Anonymous visitors cannot open preview
    Given a visitor who is not signed in
    When they open the conversation in preview
    Then access is denied

  @decision-needed
  Scenario: An admin who does not own the conversation cannot open its preview
    Given an admin with no role on the conversation
    When they open the conversation in preview
    Then access is denied

  @decision-needed
  Scenario: Moderation stays open after the conversation closes
    Given the conversation has run and is now closed
    When the moderator opens moderation for Polis 1
    Then they can still accept and reject statements
