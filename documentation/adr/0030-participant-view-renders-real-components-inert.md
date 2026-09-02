# ADR-0030: A participant view renders the real components inert, rather than framing the participant route

**Status:** proposed
**Date:** 2026-09-02
**Relates to:** [ADR-0017](0017-step-brief-slides-split-at-horizontal-rules.md), whose
"admin preview panel on the configure subtab" this replaces

## Context

Two different things in the admin are called preview.

The first is the draft Conversation at `/conversations/<id>/preview`, linked from the
conversation header. An admin walks their unlaunched Conversation as a participant would.
Steps read `preview_tool_config`, a Polis Step targets its preview poll, and the Workflow
records no progress. This meaning is deep in the code: the `[[preview]]` route segment, the
`preview_tool_config` field through the Rust DTOs, and the preview/live poll pair.

The second is `StepPreviewDialog`: a fullscreen dialog holding an `<iframe>` pointed at that
same route, with a phone/desktop toggle and a reload button, opened from the step sub-tab
strip. It was built alongside ADR-0017 to close the gap that ADR left open, namely that
typing `---` changes the participant UI and nothing in the editor says so.

The dialog has three problems and they compound.

**It always starts at the beginning.** The participant step route opens on the cover,
`phase` is component state, and no URL opens the body directly. An admin working on a Learn
step's pages clicks through the whole brief every time to reach the thing they are editing.

**It cannot see unsaved edits.** A separate document cannot read the editor's state, so
`livePreview.ts` posts the description across with `postMessage` and
`previewDraft.svelte.ts` carries that value from the Configure page up to the layout where
the button lives. Only the description crosses. Everything else needs a save and a reload,
which is why the dialog has a reload button and a line of copy apologising for it. The
value being posted is already a `$derived` in `CommonStepConfig`, reactive and correct; the
frame is the only reason it has to be serialised and thrown over a wall.

**The word is triple booked.** Draft conversation, device frame, and the in-page document
viewer in `previewKind.ts` all say preview.

## Decision

**1. Preview keeps its existing meaning. The panel is a participant view.** Nothing
renames: the route segment, `preview_tool_config`, the preview poll and the header button
all stand. The new thing takes the new name, because naming the new thing is free and
renaming the old one costs three layers. See CONTEXT.md for both entries.

**2. A participant view is inert.** No navigation, no active controls, no walking through
it. It is one screen, frozen, the way a proof is a page rather than a book.

**3. It renders the real participant components, in the admin document.** Not an iframe.
The content it shows is passed as props from the same page that authors it, so live typing
is the default rather than a bridge. `livePreview.ts` and `previewDraft.svelte.ts` are both
deleted rather than converted.

**4. It shows the whole screen, not the content alone.** `StepCover` is
`items-center justify-center` inside the `1fr` row of a `h-[100dvh]` grid; its layout is
vertical centring against a known height. Rendered loose in a panel it does not centre and
the preview is a lie. So the panel renders a device-proportioned box containing chrome bar,
content and bottom bar. The chrome earns its place beyond fidelity: its segmented fill is
derived from the slide count, so adding a slide break changes it, and that is a consequence
of the edit the admin just made.

**5. It is scoped to the surface that authors it.** Configure shows the step brief's
slides. Setup shows the Step as a participant meets it. This is what fixes the "instructions
first, page second" complaint: not a skip button, but a preview that never had the cover in
it to begin with.

**6. `StepShell` and `StepToolBody` are extracted from the participant step route.**
`StepShell` is the three-row grid, taking the chrome's props, a content snippet and a bar
snippet. `StepToolBody` is the switch from tool type to participant component, in the column
every tool body shares. The real route and the participant view both render both. A second
copy of either would diverge silently: nothing would fail, and a tool added to the route
would simply never appear in Setup's participant view.

**7. Inertness is enforced at the boundary.** The `inert` attribute plus
`pointer-events-none` on the panel container. No tool changes. Every write in every tool is
behind a user action (Prioritization on "Submit & continue", Thinking Space on save, Polis
on a vote), so blocking input blocks writes. A per-tool `preview` prop was rejected as the
safety mechanism: a prop is a convention a tool added later will not follow, and the failure
mode of forgetting it is a preview panel casting a real vote. The prop may still arrive
later as a presentation nicety, for tools that want to show sample data.

**8. Tool components receive the admin's own user id.** Reads on mount are then scoped to
someone with no answers, which yields the empty first-time state a preview should show.

**9. It is summoned from a button and opens as a full-screen overlay.** Settled after
three attempts, and the failures are the argument.

A panel docked beside the editor put the view on screen whether or not anyone was looking at
it. A right-edge drawer fixed that but was too narrow for the one thing Configure's view
exists to do: at 544px only two slides fit in a row, so a five-slide brief became a scroll. A
bottom sheet gave back the width but rationed height instead, which is the same squeeze
turned ninety degrees, and it still shared the window with a form nobody was reading while
the view was open.

The overlay has no such budget. Every screen renders near life size, and a surface with
several of them lays them out side by side. Screens are fitted to the depth under the
header, capped at life size, and the row scrolls sideways if it has to.

Summoning it also removed the split component. Pages render their editor as ordinary content
and mount the view beside it, so there is no two-column layout to own and no breakpoint below
which a panel does not fit.

**10. One screen per viewport, wherever a surface has more than one.** A step brief splits
at `---` into slides and gets a screen each. The conversation description splits at `---`
the same way, into Before you start pages, and gets a screen each too, with step zero first.
A Learn step's pages get a screen each as well. A participant reaches all of these by
scrolling or paging; a participant view cannot, because its screens are `inert` and
`pointer-events-none`, so anything past the first would be rendered and unreachable. Showing
each viewport as its own screen is both the honest rendering and the only one that works.

Which means the unit is not "the screen a participant lands on" but "every screen this
surface can show", laid out at once. That is what makes the view answerable at a glance: the
question an admin is asking on Configure is where the breaks fell, and on Setup it is
whether the pages read in order.

**11. A view shows unsaved work, not just what has been saved.** `LearnManage` reports its
pages as they stand, so the view renders the text being typed rather than the last saved
version. `LearnUI` takes the page to show as a prop, so a participant still starts at the
first page and walks from there while a view renders each page directly.

It does not track which page the editor has open. That was tried and removed: an overlay
covers the editor, so the two are never on screen together, and a selection you cannot change
while looking at the view is not worth marking. This is the price of decision 9. A docked
panel could have tracked the editor, and could not have shown the pages side by side, which
is what the view is for.

## Considered options

- **Keep the iframe, add a URL that opens the body.** Cheapest fix for the loudest
  complaint. Rejected because it leaves the `postMessage` bridge, the reload button and the
  save-then-reload loop in place, and adds a participant-facing URL parameter that exists
  only for admins.
- **Approximate the shell inside the panel.** Rejected: see decision 6.
- **Rely on autosave and invalidation instead of props.** The Configure page already
  autosaves on a debounce, so the panel could read saved data and lag by about a second with
  no plumbing at all. Rejected because the `postMessage` bridge was built precisely because
  that lag was not acceptable, and it means a server round trip per sentence typed.
- **Rename the draft concept to Draft and give the panel Preview.** The honest model: draft
  is a state, preview is a rendering. Rejected on cost, across the route segment, the Rust
  DTOs and the Polis vocabulary.

## Scope

In: a Step's Configure, a Step's Setup, and the conversation landing page. The landing page
uses a scroll-snap deck rather than the step grid, so it reuses nothing from `StepShell` and
gets its own `LandingShell`. Out: HeyForm, whose participant UI is a third-party iframe with
no component to render inert.

## Consequences

- **Editing in place is now reachable, and `inert` is what gets lifted to get there.** The
  panel already holds live editor state and real components; making a heading editable from
  the preview is a matter of relaxing the boundary for chosen elements rather than rebuilding
  the surface. That is the destination this decision is pointed at, and nothing here should
  be built in a way that blocks it.
- **The isolation the iframe gave for free is gone.** Participant styles now cascade in an
  admin document, and a participant component that grabs focus, listens on `window`, or
  assumes it owns the viewport will misbehave in a panel. `inert` covers focus. The rest is
  a real class of bug that did not exist before.
- **`StepPreviewDialog` is deleted, not gutted.** The plan was to keep it as the fullscreen
  host. In the build the dock and the fullscreen dialog show the same `screens` snippet, so
  both hosts live in `ParticipantViewSplit` and a separate dialog file would only have held
  a second copy of the device toggle. Its iframe, reload button and the copy about needing a
  save go with it.
- **Source documents are hoisted to the step layout's load.** The editor's badges and the
  participant view that renders those badges back are two consumers of one fetch, so it
  moved out of a `$effect` in `CommonStepConfig` and into `+layout.ts` under the same
  `app:documents` key the participant side uses. `CommonStepConfig` takes them as a prop.
- **`StepShell` takes the chrome's props as one object**, not eleven pass-throughs, so the
  chrome's prop list stays defined in one place. The shell still renders `StepChrome`
  itself, which is what decision 6 is protecting: a caller cannot swap it out or omit it.
- **The participant step route is refactored twice on a demo branch.** Decision 6 is two
  pure extractions with no behaviour change, but that route is the highest-risk file here
  and each wants its own commit.
- **`inert` did not cover the first component that needed covering.** `BeforeYouStart` adds
  a `deck-snap` class to `document.documentElement`, because the landing page's scroll
  container is the document. Mounted in a participant view that root belongs to the admin
  page, so it now takes an `embedded` prop that skips it. This is the mount-time side effect
  the isolation consequence above predicted, and the first use of decision 7's escape hatch.
  It arrived on the third surface, not in some distant future.
- **`StepZeroScreen` and `BeforeYouStart` are promoted out of the landing route** into
  `lib/components/participant`, since they are now rendered by admin too. Route-local until
  reused is the repo's rule and this is the reuse.
- **The landing view follows saved data, not typing.** The step brief reads one field, whose
  editor already exposes an optimistic overlay. The landing page is built from most of the
  Configure form, and every field there autosaves and then refreshes, so the view lags by
  about a second instead of updating per keystroke. Two surfaces of one feature behave
  differently, and nothing on screen says so.
- **`StepToolBody` types its step, conversation and tool config as `any`,** because the
  admin and participant routes are handed nominally different generated types for the same
  data. The tool switch carried around 23 pre-existing type errors while it lived in the
  route; moving it silenced them rather than fixing them, and the repo error count fell from
  353 to 330 for that reason alone. Giving those props real types means fixing the generated
  DTOs, and doing so will put the errors back where they can be seen.
- **A Polis participant view makes a network call per open** and shows the draft poll's real
  seed statements rather than a sample. Correct, but not free.
- **An admin who has walked their own draft sees their own data in the panel.** Preview
  writes are real for tools that store their own answers, so the empty-state property in
  decision 8 holds only until the first walkthrough, and nothing on screen explains the
  difference.
