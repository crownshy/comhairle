# PROTOTYPE — #930 Continue button placement

Throwaway. Delete this file, `PrototypeSwitcher.svelte`, and the `{#if variant}`
branches in `PrioritizationUser.svelte` once a placement is chosen.

## Question

The review screen ("Your answers") only has a Continue at the bottom, so a participant
with 9 proposals has to scroll the whole list to move on. Adding a second Continue at
the top (the literal ask in #930) leaves it floating in whitespace under a centred
header. Where should it actually go?

## How to look

Open the prioritization review screen as a participant and append `?variant=A|B|C|D`.
The floating black pill at the bottom of the screen cycles variants (arrow keys work
too). Dev builds only.

| Key | Placement | Notes from a 375px viewport |
| --- | --- | --- |
| A | Right-aligned above the list (the literal ask) | Orphaned. Sits in whitespace, attached to nothing. |
| B | Header row: heading left, button right | Wraps at mobile width, so the button drops to its own line left-aligned while the bottom one stays right-aligned. Also forces the centred header left. |
| C | Full-width under the centred header | Deliberate on mobile, keeps the header centred, but outweighs the heading and is very heavy at the 800px desktop max-width. |
| D | Sticky bar at the viewport bottom, no top button | One button, reachable at any scroll position. Solves the ticket's problem without duplicating the control. |

## Verdict

TODO — fill in which variant won and why before deleting.
