# Landing page, joining as a guest or by email, invite links and coming back later.
# Cases J-01 to J-19 from the participant journey checklist.

Feature: Getting into a conversation
  As a participant
  I want to join a conversation and find my way back to it
  So that I can take part without having to fight the sign-in

  Background:
    Given a live conversation with steps Learn, Polis, Survey, Prioritization, Thinking Space and Lived Experience
    And a visitor who is not signed in

  @smoke
  Scenario: J-01 Join as a new guest
    Given automatic anonymous login is off
    And the conversation has a short privacy policy
    When the visitor opens the conversation page
    And chooses Join as guest, then Start as new guest
    And accepts the privacy dialog
    Then a loading bar runs across the top of the page
    And a flash says they are part of the conversation
    And they land on step 1

  Scenario Outline: J-02 Dismissing the privacy dialog with <method>
    Given the conversation has a short privacy policy
    When the visitor starts joining as a guest
    And closes the privacy dialog with <method>
    Then the dialog closes
    And the Join button works again with no spinner left running

    Examples:
      | method              |
      | Escape              |
      | a click outside it  |

  Scenario: J-03 No short privacy policy skips the dialog
    Given the conversation has no short privacy policy
    When the visitor joins as a guest
    Then no privacy dialog opens
    And they land on step 1

  # Likely bug: today a new address joins straight away without proving the person owns it.
  @likely-bug @decision-needed
  Scenario: J-04 Join by email with an address that has no account
    When the visitor chooses Continue with email
    And enters an address that has never signed up
    Then they are asked for a code sent to that address before they join

  @smoke
  Scenario: J-05 Join by email with an existing account
    Given an existing email account
    When the visitor chooses Continue with email and enters that address
    Then the Check your email screen names the address
    When they enter the code from the email
    Then they are signed in and joined to the conversation

  Scenario: J-06 Wrong code, resend and change email
    Given the visitor is on the code screen for an existing email account
    When they enter a wrong code
    Then they see Invalid or expired code
    When they press Send again
    Then they are told the code was resent
    When they press Use a different email
    Then they are back at the address field
    And the code field is cleared

  Scenario: J-07 Rate limit on codes
    When the visitor requests a code 7 times in under a minute
    Then they see a Too many attempts message, not a generic failure

  @smoke
  Scenario: J-08 Returning participant signs in again
    Given a participant who joined with an email account, finished step 1 and signed out
    When they open the conversation page and sign in by email
    Then they are back in the conversation at step 2
    And they are not registered a second time
    And no second welcome flash appears

  Scenario Outline: J-09 Sign back in with a <kind> guest code
    Given a guest who joined earlier and noted their guest code
    When a new private window opens the conversation page
    And chooses Join as guest, then I have a guest code
    And enters a <kind> guest code
    Then <outcome>

    Examples:
      | kind    | outcome                                    |
      | valid   | they are back at the step they were on     |
      | invalid | they see Invalid guest ID                  |

  Scenario: J-10 Automatic anonymous login
    Given automatic anonymous login is on
    When the visitor opens the conversation page
    Then there is a single Join button
    When they press Join
    Then a guest account is created
    And they land on step 1

  Scenario: J-11 Jump back in
    Given a participant who has joined and is on step 3
    When they open the conversation page
    Then the main button reads Jump back in, or the conversation's call to action
    And pressing it takes them to step 3

  Scenario: J-12 Ended conversation
    Given the conversation has ended
    When the visitor opens the conversation page
    Then they see a closed message and a link back to the conversation list
    And there is no Join button

  Scenario: J-13 Draft conversation without preview
    Given a draft conversation
    When the visitor opens its URL without /preview
    Then they are sent to the home page

  Scenario: J-14 Email invite link while logged out
    Given an email invite was sent from Recruit
    When the visitor opens the invite link
    Then they see Login and Create account buttons
    And a line naming the invited email address
    When they sign in
    Then they come back to the invite

  Scenario: J-15 Open link that creates a guest
    Given an open link with automatic guest login
    When the visitor opens the link
    Then a guest account is created without any prompt
    And they reach the conversation page ready to join

  # Likely bug: the error branch has no workflows to render, so it may crash to a blank page.
  @likely-bug
  Scenario Outline: J-16 Invite that cannot be used: <case>
    Given <setup>
    When they open the invite link
    Then they see <message>
    And the page is not blank or broken

    Examples:
      | case             | setup                                                    | message                         |
      | expired          | an expired email invite                                  | an expired invite message       |
      | for someone else | a participant signed in who is not the invited address   | a not the intended user message |

  Scenario: J-17 Embed mode
    When the visitor opens the conversation URL with ?embed=true
    And joins and goes through two steps
    Then no site nav bar or footer shows on any page
    And every page URL keeps embed=true
    And a round help button opens Find out more

  # Likely bug: the login redirect has no backTo, so they probably land on the home page.
  @likely-bug
  Scenario: J-18 Opening a step URL while logged out
    Given the URL of a live step
    When the visitor opens it
    Then they are asked to log in
    When they log in
    Then they come back to that step

  Scenario: J-19 Skipping ahead by URL
    Given a participant on step 2
    When they paste the URL of step 5
    Then they are sent back to step 2
