# The frame around every step: progress bar, pager, step menu, language, theme and revisiting.
# Cases S-01 to S-24 from the participant journey checklist.

Feature: Step shell
  As a participant
  I want the same controls around every step
  So that I always know where I am and how to move on

  Background:
    Given a live conversation with steps Learn, Polis, Survey, Prioritization, Thinking Space and Lived Experience
    And a participant who has joined as a guest

  @smoke
  Scenario: S-01 Progress bar
    When the participant walks through Learn and Polis
    Then the progress bar has one segment per step
    And segments for earlier steps are filled
    And the current segment fills as they move through the tool

  Scenario Outline: S-02 Pager on a <setting> step before it is done
    Given the Polis step is <setting>
    When the participant opens the Polis step before voting
    Then <pager>

    Examples:
      | setting  | pager                                                     |
      | required | Next is shown disabled                                    |
      | optional | Skip is shown, and pressing it moves on to the next step  |

  Scenario: S-03 Why Next is locked on Polis
    Given the Polis step is required
    When the participant taps the locked Next before reaching the vote threshold
    Then a popover says how many more votes are needed

  # Likely bug: only Polis explains a locked Next. The other tools give no reason.
  @likely-bug
  Scenario Outline: S-04 Locked Next explains itself on <step>
    Given the <step> step is required
    When the participant taps Next before finishing <step>
    Then a popover says what is left to do

    Examples:
      | step           |
      | Thinking Space |
      | Prioritization |
      | Survey         |

  Scenario Outline: S-05 Back on the first page of a step when the previous step is <revisit>
    Given the previous step is <revisit>
    And the participant has completed it
    When they are on page 1 of the next step
    Then <back>

    Examples:
      | revisit            | back                                         |
      | revisitable        | Back takes them to the previous step         |
      | not revisitable    | Back is disabled                             |

  Scenario: S-06 Double tap Next
    When the participant double-clicks Next quickly at the end of a step
    Then a spinner shows and both pager buttons lock
    And the step is completed once
    And they move on by exactly one step

  @smoke
  Scenario: S-07 Step menu on desktop
    Given a desktop viewport
    When the participant opens the step menu
    Then a dropdown lists every step
    And completed steps show a check, passed steps that are not revisitable show a lock, and the rest show a number
    And only completed revisitable rows are clickable

  @smoke
  Scenario: S-08 Step menu on a phone
    Given a viewport under 768px wide
    When the participant opens the step menu
    Then a bottom sheet lists the same rows as the desktop dropdown
    When the viewport is resized across 768px
    Then the menu swaps between sheet and dropdown cleanly

  Scenario Outline: S-09 Light and dark from the menu on <step>
    Given the participant is on the <step> step
    When they toggle the theme from the step menu
    Then the page switches theme
    And the step menu stays open
    And <extra>

    Examples:
      | step   | extra                                         |
      | Learn  | the content repaints                          |
      | Polis  | the statement card repaints                   |
      | Survey | the survey repaints without losing answers    |

  @smoke
  Scenario Outline: S-10 Change language to <language> from the menu
    Given the participant is on the Polis step
    When they switch to <language> from the step menu
    Then the page reloads in <language>
    And the step title, pager, step menu and Polis buttons are translated
    And they stay on the Polis step

    Examples:
      | language |
      | Welsh    |
      | Gaelic   |

  Scenario Outline: S-11 Right-to-left layout in <language>
    When the participant switches to <language>
    And walks through two steps
    Then the layout is mirrored
    And the pager arrows point the other way
    And the progress bar fills from the right
    And nothing overlaps

    Examples:
      | language |
      | Arabic   |
      | Farsi    |

  Scenario Outline: S-12 <item> from the menu
    When the participant chooses <item> from the step menu
    Then Find out more opens on the <tab> tab
    And <content>

    Examples:
      | item           | tab     | content                                              |
      | FAQ            | FAQ     | the conversation FAQs show                           |
      | Privacy policy | Privacy | the tab starts with this step's data protocol text   |

  Scenario Outline: S-13 <item> link opens in a new tab
    When the participant chooses <item> from the step menu
    Then the page opens in a new browser tab

    Examples:
      | item               |
      | Terms Of Service   |
      | Cookies Settings   |

  Scenario Outline: S-14 Find out more panel on <device>
    Given the Learning Assistant is shown
    And a <device> viewport
    When the participant opens Find out more
    Then <layout>
    And the tabs are Learning Assistant, FAQ and Privacy

    Examples:
      | device  | layout                                                                     |
      | desktop | it opens from a tab on the right edge and takes half the screen            |
      | phone   | it opens from a header button and takes the full width                     |

  Scenario: S-15 Preview markers
    Given the participant is an admin previewing the conversation
    When they open the landing page in preview
    Then the preview banner shows
    When they open a step in preview
    Then a Preview pill shows instead of the banner

  @smoke
  Scenario: S-16 Revisit a finished step
    Given Learn and Polis are revisitable
    And the participant has finished Learn and Polis
    When they open Learn from the step menu
    Then Learn opens on page 1 with a full progress segment
    When they press Next to the end of Learn
    Then they go to the next revisitable step, or back to the step they were on

  Scenario: S-17 Finished step that is not revisitable
    Given Learn is not revisitable
    And the participant has finished Learn
    Then Learn shows a lock in the step menu
    When they paste the Learn step URL
    Then they are sent to their current step

  @smoke
  Scenario: S-18 Learn Back after revisit (regression)
    Given Learn is revisitable
    And the participant has finished Learn
    When they press Next on the following step's first page, then Back
    Then Learn opens on page 1
    And the pager works
    And Next is not stuck spinning

  Scenario: S-19 Sealed after finishing
    Given Allow revisit after finishing is off
    And the participant has finished every step
    When they open any step URL
    Then a flash says they have already finished
    And they land on the thank-you page

  Scenario: S-20 Slow network
    Given the network is throttled to Slow 3G
    When the participant moves between steps
    Then a thin loading bar shows at the top
    And skeletons show while content loads
    And the layout does not jump when the content arrives

  # Likely bug: completing a step checks live or not, not preview, so this records real progress and drops /preview.
  @likely-bug
  Scenario: S-21 Previewing a live conversation does not record progress
    Given the participant is an admin
    When they open the live conversation with /preview
    And finish a step
    Then no real progress is recorded for their account
    And the URL keeps /preview

  # Likely bug: in preview no step counts as completed, so Back between steps is probably disabled.
  @likely-bug
  Scenario: S-22 Back between steps in preview
    Given the participant is an admin previewing the conversation
    When they go to step 3 and press Back on its first page
    Then they go to step 2

  Scenario: S-23 Keyboard only
    When the participant moves through the step menu, the pager and each tool using only the keyboard
    Then focus is always visible
    And every control can be reached and used

  Scenario Outline: S-24 Logo in <mode>
    Given the participant is on a step in <mode>
    When they click the Comhairle mark
    Then they go to <destination>

    Examples:
      | mode    | destination                                  |
      | live    | the conversation landing page                |
      | preview | the conversation landing page under /preview |
