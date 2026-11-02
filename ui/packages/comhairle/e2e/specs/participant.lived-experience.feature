# Three videos, then recording your own story. No admin setup.
# Cases X-01 to X-09 from the participant journey checklist. Recording needs a fake camera
# and microphone (Chromium's fake media stream flags) or a permission that is denied.

Feature: Lived Experience step
  As a participant
  I want to watch other people's stories and record my own
  So that my experience is heard alongside theirs

  Background:
    Given a live conversation with steps Thinking Space and Lived Experience
    And a participant who has joined as a guest and reached the Lived Experience step

  @smoke
  Scenario: X-01 Walk the screens
    When the participant presses Next through the intro, three videos, recording and done screens
    Then each video screen shows Video X of 3
    When they go back to the first video and press Back
    Then they are on the intro screen

  Scenario: X-02 Video playback
    When the participant plays each video with sound
    Then the video controls work
    And no video autoplays

  # Likely bug: two video elements share one reference, so the live preview may not show.
  @likely-bug
  Scenario: X-03 Record
    Given camera and microphone access is allowed
    When the participant opens the recording screen
    Then a live camera preview shows before recording
    When they start recording
    Then a 20-second countdown runs
    And Stop Now stops the recording

  # Likely bug: no error handling, so they get a black box and Start Recording does nothing.
  @likely-bug
  Scenario: X-04 Camera permission denied
    Given camera access is denied
    When the participant opens the recording screen
    Then a message explains that camera access is needed
    And they can still move on

  # Likely bug: Upload does not upload anything.
  @likely-bug
  Scenario: X-05 Upload
    Given the participant has recorded a clip
    When they press Upload
    Then the recording is uploaded
    And they move on to the done screen

  # Likely bug: uses old Svelte click syntax and probably does nothing.
  @likely-bug
  Scenario: X-06 Record again
    Given the participant has recorded a clip
    When they press Record Again
    Then the old clip is discarded and the live preview shows again

  # Likely bug: the camera stream is not stopped.
  @likely-bug
  Scenario Outline: X-07 Leaving the recording screen with <button> stops the camera
    Given the participant is on the recording screen without recording
    When they press <button>
    Then the camera stream is stopped

    Examples:
      | button |
      | Back   |
      | Next   |

  # Likely bug: the bar stops at 5/6.
  @likely-bug
  Scenario: X-08 Progress on the done screen
    When the participant reaches the done screen
    Then the progress segment is full

  Scenario: X-09 Reload
    Given the participant is on video 2
    When they reload the page
    Then they start again at the intro screen
