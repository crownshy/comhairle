# CrownShy Style Guide

## Introduction

This guide outlines agreed-upon principles for how we should both write and maintain
code at CrownShy. It exists to help new engineers (and LLM agents) get up to speed
faster, provides a common set of standards to uphold in PR reviews, and reduces the
low-level decisions that slow us down.

This is a living document and does **not** exist as a hard set of rules. Anyone on the
team can propose a change to add new conventions or change existing ones. Please keep it
up to date so we have a single source of truth.

**This guide covers:**

- General principles (language-agnostic)
- Rust (backend)
- JavaScript + Svelte (frontend)

**How to use it:**

- Reference it when you're unsure how to approach something.
- Link to it in PR comments instead of re-explaining the same feedback.
- Treat it as a starting point, not a rulebook, and use personal judgement when
  circumstances demand it.
- **When the same feedback comes up a second time in review, add it here.** The trigger
  for a new entry is not "is this important" but "have we now said this twice". That keeps
  the guide grounded in real friction rather than speculation.

Related docs: domain language lives in [CONTEXT.md](CONTEXT.md); past architectural
decisions in [documentation/adr](documentation/adr); the agent entry point is
[AGENTS.md](AGENTS.md).

### A note on LLM-generated code

Using LLMs in development is not discouraged at CrownShy. However, LLM-generated code
comes with problems worth naming: models are trained to produce working code, not code
that fits your codebase's conventions. Generated code is often terse, inconsistently
named, and written without awareness of the patterns we've established here.

The consequence is that copy-pasting LLM output directly into the codebase is an
antipattern. It shifts the burden of review onto your colleagues.

Instead, treat generated code the way you'd treat code lifted from Stack Overflow or an
unfamiliar source: read it, understand it, and rewrite it in the style outlined in this
document. If you wouldn't submit code as-is without an LLM having written it, you
shouldn't submit it because an LLM wrote it either. The bar is the same.

## General Principles

These apply regardless of language or layer.

### Clarity

Write code for the person reading it next, not the person writing it now. A solution
that is slightly longer but immediately understandable is almost always better than a
compact one that requires more decoding.

**Things that reduce clarity and should be avoided:**

#### Nested ternary operators

These are harder to read and reason about than an `else if` block. "Ternary" also means
three, and should be made up of three parts: a condition, the snippet to run if the
condition is met, and the fallback. It is meant to be shorthand for a simple if/else
block, not a longer if / else if / else if / else chain. If a code block contains
multiple conditions, `else if` blocks are much easier to reason about.

#### Abbreviations in variable names

In general, longer variable names are much clearer for developers reading the code than
abbreviations. Prefer full words: `conversation` not `conv`, `moderationStatus` not
`modStatus`, `index` not `idx`, `denominator` not `denom`. Widely understood initialisms
(`url`, `id`, `api`, `html`) are fine. An exception is closures and callback functions
where the abbreviation is clear from the variable the closure is tied to.

Define domain jargon at first use. Project-specific shorthands (`aux` for
`PolisStatementAux`, `tid` for a Polis statement id) are invisible to a reviewer who
doesn't live in that subsystem. Drop a one-line `//` gloss the first time one appears in
a file, or prefer the spelled-out name.

#### Deep nesting

Deeply indented code is hard to follow, so keep nesting shallow:

- If a block nests more than about four levels deep, pull the inner work out into its own
  well-named function.
- Prefer **guard clauses (early returns)** over wrapping the body in an `if`.
- **Invert** to handle the negative and error cases first, so the rest of the function
  reads as a straight, uninterrupted run of the successful path.

## Rust (backend)

Conventions for `api/`, `data_model/`, and `adaptors/`. See `api/src/tools.rs` for the
house style, and use rustdoc (`///` on items, `//!` for module-level docs).

_This section is a work in progress. Add conventions as they are agreed._

### Name the update-payload method `to_values`

In a model, the method that turns an update/insert struct into the
`Vec<(SomeIden, SimpleExpr)>` of `(column, value)` pairs for a sea-query statement is
called `to_values`. This is the established name across the models (`conversation`,
`event`, `workflow_step`, `media`, and others), so reach for it rather than coining a
per-model alternative. Reading it at a glance is easier when every model spells it the
same way.

## JavaScript + Svelte (frontend)

The day-to-day working agreement for the SvelteKit app. The app lives in the pnpm
workspace at **`ui/packages/comhairle`**; paths below are relative to that package unless
noted. This is a **Svelte 5 runes** codebase.

### Keep files small and single-purpose

Route files (`+page.svelte`) are **composition roots**, not dumping grounds. If a page
grows past ~300 lines it's a smell; past ~500 it's a bug in how it's split.

- **Presentation** → `.svelte` components. **Colocate by default**: a component used by a
  single route lives next to that route (e.g.
  `conversations/[conversation_id]/design/AddStepDialog.svelte`), matching the existing
  pattern (`media-library/DeleteDialog.svelte`, `events/[event_id]/AgendaEditor.svelte`).
  Promote to `src/lib/components/**` only once a component is (or is clearly about to be)
  reused across routes; `src/lib/components/ui/**` stays reserved for shadcn primitives.
  Router-reserved filenames (`+page`, `+layout`, `+error`) are the only `.svelte` files a
  route folder treats specially, so a plain colocated component is safe there.
- **Reactive state / orchestration** → a `.svelte.ts` runes module (colocated next to its
  feature, or under `src/lib/stores/**` / `src/lib/tools/**`). See
  `notifications.svelte.ts` and `sidebarWidth.svelte.ts` for the pattern.
- **Pure logic** (parse, format, match, transform, build-a-thing) → a plain `.ts` under
  `src/lib/utils/**`, with a colocated `*.test.ts`.

A `+page.svelte` should mostly _wire_ these together and branch between screens.

#### Reporting work (upcoming)

Report UI is being built out soon, across multiple tools (`thinking_space`, `polis`, and
others). A couple of conventions to settle before that lands:

- **Group report components under `report/`, not under each tool.** Rather than scatter
  report UI as `components/tools/<tool>/report` (or `tools/<tool>/report`), group it under
  `src/lib/components/report/<tool>` (e.g. `report/thinking_space`, `report/polis`), with
  general-purpose report pieces sitting directly in `report/`. That keeps both the
  tool-specific and the shared report components scoped to reporting in one place.
  `src/lib/components/report/` already exists, and the flat `components/polis-report/`
  would fold into `report/polis` under this scheme. **Status: proposed, still open for
  discussion.**
- **This lives in tension with colocate-by-default, and is resolved the usual way.** A
  report component used by a single route still colocates next to that route; it graduates
  to `components/report/**` once it is shared across routes or is clearly a general
  reporting primitive.

### Pure and testable by default

- A pure helper takes its inputs as **arguments** and returns a value. It must not reach
  into runes state, `page.url`, `sessionStorage`, or the DOM. Thread those in as params
  instead, that's what makes it testable.
- Colocate unit tests (`foo.ts` + `foo.test.ts`) and run `pnpm test:unit`. Vitest is set
  up; `src/lib/utils/urlValidation.test.ts` is the reference style.
- When extracting from a component, split the **pure core** from the **I/O shell**: the
  pure, tested function behind a thin async that just loads data and calls it.

### Reuse before you build

Before hand-rolling UI or a helper, **check what already exists**: grep
`src/lib/components`, `src/lib/components/ui` (shadcn-svelte primitives), and
`src/lib/utils`.

- Buttons → the shadcn `Button` (`$lib/components/ui/button`), incl. `href` links and
  `loading-button` for pending states.
- Dialogs, selects, tables, command palette, skeletons, sonner toasts all live in
  `src/lib/components/ui/**`. Don't re-roll a shadcn primitive.
- Class merging → `cn()` from `$lib/utils`. Never concatenate class strings by hand.
- Icons → `lucide-svelte`. Don't inline bespoke SVGs for common glyphs.
- Never hand-roll what a proven library already does: charts → LayerCake / layerchart /
  `@carbon/charts-svelte`; rich text → TipTap / Carta; QR → `svelte-qrcode`;
  drag-and-drop → the existing dnd deps. Add/use a dep before writing bespoke code.

If you copy a block a second time, stop and extract it.

### Naming

- **Spell it out.** Prefer full words for variables, functions, props, and types (see
  [Abbreviations in variable names](#abbreviations-in-variable-names) under General
  Principles, which applies here too).
- **Name the props type; don't inline the annotation.** Declare a `type Props = { … }`
  (or `interface Props`) above the destructure and annotate with it, rather than inlining
  a large object literal after `}:`.

    ```svelte
    <!-- WRONG: a big inline object literal buried in the destructure -->
    let { row, selected, onToggle }: { row: Foo; selected: boolean; onToggle: () => void } =
    	$props();

    <!-- RIGHT: a named Props type, destructure stays scannable -->
    type Props = {
    	row: Foo;
    	selected: boolean;
    	onToggle: () => void;
    };
    let { row, selected, onToggle }: Props = $props();
    ```

- **Derive types; don't restate them.** Prefer extracting or deriving a type from its
  source of truth over re-declaring a matching shape by hand: a hand-copied duplicate
  drifts, and it moves errors to the wrong place. To type a value you pass into a
  component, reach for `ComponentProps` so a change to the component's prop type lands as
  an error _on the value_, not as an ambiguous "incorrect props" error down at the markup.

    ```svelte
    <script lang="ts">
    	import type { ComponentProps } from 'svelte';
    	import ComponentA from './ComponentA.svelte';

    	// If ComponentA's `list` prop type changes, the error surfaces *here*, on the value…
    	let list: ComponentProps<typeof ComponentA>['list'] = ['a', 'b', 'c'];
    </script>

    <!-- …not as a vague props error down here on the usage. -->
    <ComponentA {list} />
    ```

### Imports

- **`$lib` alias for cross-feature imports; relative paths for colocated files.** Reach
  into shared code (`$lib/components/…`, `$lib/utils/…`) through the `$lib` alias rather
  than long `../../..` chains. A component importing its own siblings (a colocated child,
  its `*.svelte.ts`, its `*.test.ts`) uses a plain relative path (`./Child.svelte`). Rule
  of thumb: crossing out of the current feature folder → `$lib`; staying inside it →
  relative.
- **Import ordering is not automated.** Prettier is our only formatter and it does not
  reorder imports; there is no import-sort plugin in the toolchain today. Keep imports
  tidy by hand (external packages, then `$lib`, then relative). **Open question: adopt a
  prettier import-sort plugin so this stops being manual?**

### Error handling

- Prefer **`tryCatchAsync`** (`$lib/utils/errorHandling`) over raw `try/catch`.
- **Never destructure its `Result`**, it breaks TypeScript's discriminated-union
  narrowing. Keep the object and branch on `.err`:

    ```ts
    const res = await tryCatchAsync(() => apiClient.doThing());
    if (res.err !== null) return handle(res.err);
    use(res.ok);
    ```

- Client-side data goes through `apiClient` (`@crownshy/api-client/client`); server-side
  loads use `tryFetch` from the same util.

### Comments

- Use **hoverable doc comments** on anything exported. Editors surface these on hover, so
  they earn their keep; reserve plain `//` for short inline notes on a tricky line.
    - **TypeScript** → TSDoc on functions, component props, and non-obvious types.
      `errorHandling.ts` and `urlValidation.ts` show the house style (`@param`, a one-line
      summary). Mind the delimiter: a doc comment must open with a two-star delimiter. A
      single-star block comment looks the same in source but editors do NOT surface it on
      hover, so a prop documented that way reads as undocumented at the call site. On any
      exported item or interface member, use the two-star doc delimiter or nothing.
    - **Rust** → rustdoc (`///` on items, `//!` for module-level docs), the same as the
      rest of `api/`, `data_model/`, and `adaptors/`.
- **Comment the _why_, not the _what_.** A comment earns its place only when it carries
  something the reader can't recover from the code: a non-obvious reason, a footgun, a
  constraint, a link to context. Delete comments that restate the line.
    - Good: `// -ml-3.5 cancels its own px-3.5 so the icon aligns to the gutter column`.
    - Good: `// $derived (not $state + $effect) so SSR renders the real order too`.
    - Noise: `// derived value for the reordered steps` above `let reorderedSteps = $derived(...)`.
- **State each rationale once.** If the same "why" is true in three files, explain it
  fully in the canonical spot (usually the exported helper's TSDoc) and have the others
  point to it by name rather than restating the paragraph.
- **Density follows surface.** Exported functions, props, and non-obvious types get a
  hoverable doc comment. Inside a function body, prefer self-explaining names and small
  functions over inline narration.
- No em dashes (or long dashes) in comments or prose. Use commas, parentheses, or a full
  stop.

### Styling

- **Tailwind v4 utilities, inline.** No custom classes in `<style>` blocks: if you need
  CSS, it's a utility. (Complex keyframe/`@container` work is the rare exception and must
  be justified in a comment.)
- **Never smaller than `text-base` for content.** `text-sm` only for small labels;
  nothing below `text-sm`, ever. This is enforced on desktop.
- Use the **flat shadcn design tokens** (`bg-card`, `text-muted-foreground`, …). When
  translating from Figma, map its doubled token names to the flat token in markup.

### Svelte (runes only)

State is `$state` / `$derived` / `$props`, not the Svelte 4 style. Do not write
`export let`, `$:` reactive statements, `$$props` / `$$restProps`, or default to writable
stores for component state. If you see those in a file you're editing, they're legacy,
migrate them.

- **`$derived` is the default for computed state. `$effect` is a last resort.** If a value
  is a function of other state (props from `load` included), it's a `$derived`. Reserve
  `$effect` for true side effects that reach _outside_ the reactive graph: DOM
  measurement, event listeners, subscriptions, logging, imperative third-party calls.
- **Never use `$effect` to keep one piece of state in sync with another.** This is the
  single most common runes mistake (and what LLMs emit by default), so call it out in
  review. In Svelte 5 a `$derived` is _writable_: a local edit (e.g. a `bind:value` the
  user types into) overrides it until a dependency changes, then it resyncs on its own.
  That is exactly the "seed from a prop, re-seed on change" behaviour, with no effect:

    ```svelte
    <!-- WRONG: state mirrored from a prop via an effect (stale-value + extra-render footgun) -->
    let topicInput = $state(topic);
    $effect(() => {
    	topicInput = topic;
    });

    <!-- RIGHT: a writable derived. bind:value still works; it resyncs when `topic` changes -->
    let topicInput = $derived(topic);
    ```

    If you genuinely can't express it as a `$derived` (rare), that's the signal to stop
    and rethink the data flow, not to reach for `$effect`.

- **Reach for `onMount` when the setup runs once and doesn't track reactive state.** Both
  are Svelte 5 (`onMount` / `onDestroy` are still first-class), so this is a readability
  call, not a legacy one. `onMount` says "wire this up once, on mount, and tear it down on
  unmount"; `$effect` says "this re-runs whenever its dependencies change". Use that to
  signal intent:
    - **One-time, non-reactive setup → `onMount`.** Attaching a `window`/`document` event
      listener, kicking off a one-shot subscription, an imperative third-party init. Return
      a cleanup function for the teardown. A reader doesn't have to hunt for what makes it
      re-run, because nothing does.
    - **Setup that must re-run when reactive state changes → `$effect`.** The effect body
      reads runes state and needs to re-subscribe / re-measure / re-sync when it changes.
      Here the re-run is the point.
    - Rule of thumb: if the body reads no reactive state (or only reads it lazily inside a
      callback that fires later, like an event handler), it isn't tracking anything, so
      `onMount` states the intent more honestly than an `$effect` that never re-runs. See
      `unsavedChangesGuard.svelte.ts` for the pattern (listener wired once; the dirty check
      runs at event time via a getter).

### SvelteKit

- Use `depends()` in `load` functions to declare explicit cache keys for invalidation.
- When sibling pages fetch the same resource, hoist the fetch to the nearest shared layout
  `load` and read it via `await parent()` in children.

### Before you finish

- `pnpm test:unit` and `pnpm check` (svelte-check) pass.
- Formatting is not optional, but **scope it to the files you touched**: run
  `pnpm prettier --write <files>` (or `pnpm exec prettier --write $(git diff --name-only)`),
  not the repo-wide `pnpm format`. A bare `pnpm format` reformats hundreds of unrelated
  files and buries your real diff, drowning reviewers. Run `pnpm lint` (eslint) to check.
- No `: any` where a real type fits; no stray `console.*` left in.
- **Do not `git commit`.** Leave changes staged/unstaged for the human to review and
  commit themselves.
