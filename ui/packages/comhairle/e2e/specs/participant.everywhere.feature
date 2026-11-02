# Checks that cut across every step. Each one walks the whole journey, so the matching
# test should reuse the per-step helpers from the other participant specs.
# Cases C-01 to C-05 from the participant journey checklist.

Feature: Whole participant journey
  As a participant on any device, theme or language
  I want every step to work the same way
  So that how I take part does not limit what I can say

  Background:
    Given a live conversation with steps Learn, Polis, Survey, Prioritization, Thinking Space and Lived Experience
    And a participant who has joined as a guest

  @smoke
  Scenario Outline: C-01 Whole journey on a phone in <browser>
    Given a phone viewport in <browser>
    When the participant goes through every step to the thank-you page
    Then nothing is cut off
    And the page never scrolls sideways
    And the pager can always be reached

    Examples:
      | browser                      |
      | WebKit (iOS Safari)          |
      | Chromium (Android Chrome)    |

  Scenario: C-02 Whole journey in dark mode
    Given dark mode
    When the participant goes through every step
    Then all text is readable
    And the embedded tools match the dark theme

  Scenario Outline: C-03 Whole journey in <language>
    Given the participant has switched to <language>
    When they go through every step
    Then no English strings are left on any page

    Examples:
      | language |
      | Welsh    |
      | Gaelic   |

  Scenario: C-04 Screen reader names
    When the participant is on a step
    Then the progress bar announces Step X of Y
    And every button has an accessible name
    And popovers are exposed to assistive technology when they open

  Scenario: C-05 Two tabs on the same step
    Given the participant has the same step open in two tabs
    When they finish the step in the first tab
    And act on the step in the second tab
    Then the second tab moves them on or explains the step is already done
    And no error toasts show
