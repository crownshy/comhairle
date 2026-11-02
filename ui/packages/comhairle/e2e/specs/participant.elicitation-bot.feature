# The older chat step, replaced by Thinking Space. Only worth automating while a live
# conversation still uses it.
# Cases E-01 to E-06 from the participant journey checklist.

Feature: Elicitation bot step
  As a participant
  I want to talk through a topic with the bot and approve the claims it draws out
  So that my views are captured in statements I agree with

  Background:
    Given a live conversation with an Elicitation bot step
    And a participant who has joined as a guest and reached the Elicitation bot step

  Scenario: E-01 Chat
    When the participant opens the step
    Then an opening message from the topic shows
    When they use the suggested question
    Then the reply streams in
    When they type their own message and press Enter
    Then it is sent and the reply streams in

  # Likely bug: the first message of a new session is probably hidden.
  @likely-bug
  Scenario: E-02 First message
    Given a fresh chat session
    When the participant sends one message
    Then their message shows in the chat

  Scenario: E-03 Claims
    Given the bot has drawn out some claims
    When the participant approves one claim
    Then Next and Continue unlock
    And they can edit a claim, remove a claim and add their own

  # Likely bug: claims are only stored in the browser.
  @likely-bug
  Scenario: E-04 Claims on another device
    Given the participant has approved a claim
    When they open the step on another device
    Then the approved claim is there

  # Likely bug: it still shows as approved after editing.
  @likely-bug @decision-needed
  Scenario: E-05 Edit an approved claim
    Given the participant has approved a claim
    When they edit it
    Then it goes back to waiting for approval

  Scenario: E-06 Phone tabs
    Given a phone viewport
    When new claims arrive while the participant is on the Chat tab
    Then a dot shows on the Claims tab
    And they can switch between Chat and Claims
