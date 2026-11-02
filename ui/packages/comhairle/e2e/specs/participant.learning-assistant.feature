# The question-and-answer chat built on the uploaded documents, shown under Learn and in Find out more.
# Cases A-01 to A-12 from the participant journey checklist.

Feature: Learning Assistant
  As a participant
  I want to ask questions about the background documents
  So that I can check what they say without reading every page

  Background:
    Given a live conversation with steps Learn and Polis
    And one PDF has been uploaded to the Learning Assistant and has finished parsing
    And Show Learning Assistant is on
    And a participant who has joined as a guest

  @smoke
  Scenario Outline: A-01 Learning Assistant is <state> when the toggle is <toggle>
    Given Show Learning Assistant is <toggle>
    When the participant opens the Learn step and Find out more
    Then the Learning Assistant is <state> in both places
    And no empty box is left behind

    Examples:
      | toggle | state   |
      | on     | visible |
      | off    | hidden  |

  @smoke
  Scenario: A-02 Ask a question
    When the participant asks a question the PDF answers
    Then Finding answer shows
    And the answer streams in
    And the input is locked until the answer has finished

  Scenario: A-03 Citations
    Given an answer with a citation marker
    When the participant hovers the marker
    Then a short preview of the source shows
    When they click the marker
    Then the PDF opens at the highlighted passage

  Scenario: A-04 Sources list
    Given an answer that cites the PDF
    Then the Sources under the answer show one chip per document
    And each chip opens the document viewer

  Scenario: A-05 Follow-up question
    Given the participant has asked a question and got an answer
    When they ask a follow-up that only makes sense with the previous answer
    Then the answer uses the earlier context

  Scenario: A-06 History after reload
    Given the participant has asked two questions
    When they reload the page
    Then both questions are listed, newest first and expanded
    And their citations still open at the highlighted passage

  Scenario: A-07 Shared between Learn and the panel
    Given the participant has asked a question under Learn
    When they open Find out more
    Then the same question and answer show there

  # Likely bug: panel questions are sent with a [Reading "Step name"] prefix that may show in history.
  @likely-bug
  Scenario: A-08 Question asked from the panel reads cleanly in history
    Given the participant is on the Polis step
    When they ask a question in Find out more
    And reload the page
    Then the question in history reads exactly as they typed it

  Scenario: A-09 Error and retry
    Given the participant is offline
    When they ask a question
    Then an error box shows with Try again
    When they come back online and press Try again
    Then the question is answered

  Scenario: A-10 Citations on a phone
    Given a phone viewport
    And an answer with a citation marker
    When the participant taps the marker
    Then the document viewer opens and is usable
    And no hover preview step is needed

  Scenario: A-11 Re-synced Learn content
    Given the participant has an answer with a citation
    And the admin has synced the Learn content again
    When the participant opens that citation
    Then a plain message says the source is no longer available

  Scenario Outline: A-12 Ask in Welsh with cross-language search <setting>
    Given Cross-language Search is <setting>
    When the participant switches to Welsh and asks a question the PDF answers
    Then a sensible answer comes back

    Examples:
      | setting |
      | on      |
      | off     |
