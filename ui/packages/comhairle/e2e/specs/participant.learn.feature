# Rich content pages. The admin palette calls this step Rich content page.
# Cases L-01 to L-13 from the participant journey checklist.

Feature: Learn step
  As a participant
  I want to read the background material page by page
  So that I understand the topic before I give my views

  Background:
    Given a live conversation whose first step is a Learn step with 3 pages
    And the pages contain an image, a wide table, a YouTube embed and a PDF source badge
    And a participant who has joined as a guest

  @smoke
  Scenario: L-01 Page through a multi-page Learn
    When the participant presses Next through all pages
    Then each page shows Page x of 3
    And each page starts scrolled to the top
    And the progress segment fills a little more on each page
    And Next on the last page finishes the step

  Scenario: L-02 Single page
    Given the Learn step has one page
    When the participant opens it
    Then no page counter shows
    And Next finishes the step

  Scenario: L-03 Back inside Learn
    Given the step before Learn is revisitable and completed
    When the participant presses Back on page 2
    Then they are on page 1
    When they press Back again
    Then they go to the previous step

  Scenario: L-04 Images
    When the participant clicks an image
    Then a lightbox opens with zoom
    And it closes again
    When they click an image that is wrapped in a link
    Then the link is followed instead of the lightbox opening

  Scenario: L-05 Video embed
    When the participant plays the YouTube embed
    Then it plays and can go fullscreen
    When they open a page with a video from a host that is not on the allowed list
    Then a blocked video message shows in its place

  Scenario: L-06 Audio
    Given a page with an audio clip
    When the participant opens the page
    Then the clip shows in the native player and does not autoplay
    And it plays when they press play

  Scenario Outline: L-07 Wide table on a phone in <theme> mode
    Given a phone viewport in <theme> mode
    When the participant opens the table page
    Then the table scrolls sideways inside itself
    And the page itself does not scroll sideways
    And the cell colours stay readable

    Examples:
      | theme |
      | light |
      | dark  |

  Scenario: L-08 Glossary terms
    Given a glossary term that is used on the page
    Then the term has a dotted underline
    When the participant hovers it
    Then a tooltip shows its definition
    When they move focus to it with Tab
    Then the tooltip shows as well

  Scenario Outline: L-09 Source document badge for a <type> file
    When the participant clicks a <type> source badge
    Then <outcome>

    Examples:
      | type  | outcome                                                     |
      | PDF   | the PDF viewer opens with zoom, page arrows and Download    |
      | .docx | the document viewer opens                                   |
      | .txt  | the text viewer opens                                       |
      | .xlsx | the file downloads                                          |

  Scenario: L-10 Source not parsed yet
    Given a badge for a document that has not finished parsing
    Then the badge reads Source document
    When the participant clicks it
    Then the file downloads

  @decision-needed
  Scenario: L-11 Page missing in your language
    Given one Learn page has no translation in Welsh
    When the participant switches to Welsh and opens that page
    Then a message says the page is not available in this language
    And the message itself is in Welsh

  # Likely bug: with zero pages Next keeps counting up and never completes the step.
  @likely-bug
  Scenario: L-12 Learn step with no pages
    Given a Learn step with every page deleted
    When the participant opens it and presses Next
    Then the step completes

  Scenario: L-13 Skeletons
    Given a slow network
    When the participant moves into the Learn step
    Then a skeleton shows while the first page loads
    When they move between Learn pages
    Then no skeleton shows
