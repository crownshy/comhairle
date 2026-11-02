# Event pages, registering, the lobby and the live call.
# Cases V-01 to V-10 from the participant journey checklist. The call scenarios need a
# second browser context playing the host.

Feature: Events and live calls
  As a participant
  I want to find, register for and join the conversation's events
  So that I can take part in live sessions

  Background:
    Given a live conversation with an upcoming event
    And a participant who has joined with an email account

  Scenario: V-01 Event list
    Given the conversation also has a live event and a past event
    When the participant opens the conversation's events page
    Then each event shows a Live, Upcoming or Past badge
    And each shows its date and time
    And each shows attendees out of capacity

  Scenario: V-02 Event page logged out
    Given a visitor who is not signed in
    When they open the event page
    Then they are asked to log in by email
    When they log in
    Then they come back to the event page

  @smoke
  Scenario: V-03 Register
    When the participant presses Register to Attend
    Then the button reads Registering
    And then You're registered

  # Likely bug: the status does not update until the page is reloaded.
  @likely-bug
  Scenario: V-04 Start time passes
    Given the participant is registered and has the event page open
    When the start time passes
    Then Join meeting appears without a reload

  Scenario Outline: V-05 Event invite link while <state>
    Given an event invite link
    And the participant is <state>
    When they open the invite link
    Then <outcome>

    Examples:
      | state      | outcome                                                                |
      | logged in  | a registered toast shows and they land on the event page               |
      | logged out | they are signed in first, then registered and shown the event page     |

  @smoke
  Scenario: V-06 Lobby and joining
    Given the participant is registered
    When they open the lobby before the host has started
    Then they see Waiting for the host
    When the host starts the meeting
    Then they are moved into the call with their microphone muted
    And the agenda is visible

  # Likely bug: the indicator is always on.
  @likely-bug
  Scenario: V-07 Recording indicator
    Given a call that is not being recorded
    When the participant joins it
    Then no Recording indicator shows

  Scenario: V-08 Breakout rooms
    Given the participant is in the call
    When the host opens breakout rooms
    Then the participant moves to their room
    And sees the room's question and timer
    When the host ends the breakouts
    Then the participant gets a 5-second warning
    And returns to the main call automatically

  # Likely bug: the ended screen may flash and then switch to the lobby.
  @likely-bug
  Scenario: V-09 Meeting ends
    Given the participant is in the call
    When the host ends the meeting
    Then the participant sees the meeting ended screen and stays on it

  Scenario: V-10 Call on a phone
    Given a phone viewport
    When the participant joins the call
    Then the agenda shows as chips with arrows
    And the breakout panel opens as a bottom sheet
