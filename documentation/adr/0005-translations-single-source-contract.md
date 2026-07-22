# ADR-0005: Translatable fields are driven by a single `TranslationSource` contract

**Status:** testy test; to be reviewed
**Date:** 2026-07-21

## Context

Every translatable field in the admin UI (step name / description, the seven conversation
[configure](../../ui/packages/comhairle/src/routes/(admin)/admin/conversations/[conversation_id]/configure/+page.svelte)
fields, event fields, prioritization proposals, and learn pages) renders through one component,
[TranslatableField.svelte](../../ui/packages/comhairle/src/lib/components/Translation/TranslatableField.svelte).
That component had grown **two entirely different personalities**, chosen at runtime by whether a
`translation` prop was passed:

- **textContent mode** (`translation` present): the field saves itself via
  `saveTranslation()` against a `TextContent` entity, then `invalidateAll()`, and shows a
  "Saving / Saved" indicator.
- **callback mode** (no `translation`): the field delegates persistence to `onSaveSource`,
  `onSaveTarget`, `onAiTranslate`, `onApprove`, `onMarkAsDraft` props supplied by the parent, and
  showed **no** save indicator.

A single `isTextContentMode` flag forked the code in roughly eight places (`badges`,
`editorContents`, `editorStatuses`, the inline debounce, and all five dialog handlers). The two
data shapes (a `TextContent` with per-locale rows, versus learn's inline
`ExtendedLocalizedPage[]` packed into the step `tool_config`) were bridged inside the component by
re-deriving "contents and statuses per locale" twice.

This muxing was the direct cause of several defects:

- **No save indicator on the learn / setup step**, because the indicator state was only ever set in
  the textContent branch. Passing `translation` "fixed" the indicator but rerouted persistence to
  the wrong backend (learn is not `TextContent`-backed), so the two could never both be satisfied.
- **Content overwritten when switching learn pages quickly.** The component's own 1s inline debounce
  fired `onSaveSource` after the page had changed, writing the previous page's text into the current
  page. This was papered over with a `canSwitchPage` flag that blocked page switching for 1s after
  every keystroke (fixed later; see below).
- **A reload per keystroke.** A wrapper in learn's page controller dropped its `invalidate`
  argument, so every autosave ran `invalidateAll()`.
- **The dialog editor
  ([TranslationEditor.svelte](../../ui/packages/comhairle/src/lib/components/Translation/TranslationEditor.svelte))
  kept its own copy** of contents and statuses and debounced saves at 500ms, while the field
  debounced at 1000ms: two stateful copies of one datum, kept in sync by two debounces.

An earlier pass on the learn tool already moved page state into a
[Pages](../../ui/packages/comhairle/src/lib/tools/learn/Pages.svelte.ts) runes class and gave it an
observable, flushable save state machine, which removed the `canSwitchPage` lockout. This ADR
generalises that direction into a single contract shared by every translatable field.

The deepest cause is a data-model split (some translatable content is a first-class `TextContent`
entity, some is inline JSON in `tool_config`). Unifying that at the backend is the eventual ideal
and would delete this whole category of frontend complexity, but it needs a schema migration and is
out of scope here. This ADR is the frontend answer that stops the split from leaking into the
component layer.

## Decision

**Introduce one `TranslationSource` contract that owns both reading and persisting a field's
translations, and reduce `TranslatableField` (and `TranslationEditor`) to dumb views over it.**

```ts
type TranslationSource = {
	// reads, exposed as reactive getters
	get contents(): Record<string, string>; // per-locale content, latest edit reflected immediately
	get statuses(): Record<string, TranslationStatus>; // per-locale draft / approved / primary
	get saveState(): SaveState; // 'idle' | 'saving' | 'saved' | 'error'
	// writes; saveSource / saveTarget are debounced (driven by typing),
	// the rest are immediate (discrete actions)
	saveSource(content: string): void;
	saveTarget(locale: string, content: string): void;
	aiTranslate(locale: string, source: string): Promise<{ content: string; requiresValidation: boolean }>;
	approve(locale: string): Promise<void>;
	markAsDraft(locale: string): Promise<void>;
	flush(): Promise<void>; // commit any pending debounced save and await the in-flight one
};
```

- **The source owns reads and writes.** Deleting the `isTextContentMode` fork requires abstracting
  both halves; abstracting only saves would leave the read fork in place. The component reads
  `source.contents` / `source.statuses` and calls `source.saveSource(...)`, with zero mode
  awareness.
- **The source owns the persistence machinery** (debounce timing, `saveState`, `flush`), not the
  component. `TranslatableField` and `TranslationEditor` therefore lose their debounces, their save
  status juggling, the dialog's duplicate `contents` / `statuses` copy, the `onRegisterFlush`
  handshake, and the `saveStatus` prop.
- **Two implementations:**
  - `createTextContentSource(getTranslation, primaryLocale, supportedLanguages)` in
    [translationSource.svelte.ts](../../ui/packages/comhairle/src/lib/components/Translation/translationSource.svelte.ts).
    It takes a **getter thunk** `() => translation` (so it tracks the live prop across
    `invalidateAll()` instead of snapshotting it), derives `contents` / `statuses` with `$derived`
    over a **thin optimistic overlay** of just-typed edits, saves through the existing
    `translationUtils` helpers, and keeps today's `invalidateAll()` after each save so the server
    stays the source of truth.
  - `createLearnSource(pages, ...)` in
    [createLearnSource.svelte.ts](../../ui/packages/comhairle/src/lib/tools/learn/createLearnSource.svelte.ts),
    a thin adapter projecting the **current page** of the `Pages` collection onto the contract.
    `Pages` stays a page-collection controller and does not learn about the contract.
- **One reactivity rule for both:** `contents` always reflects the latest keystroke immediately.
  learn holds it in its in-memory model; textContent holds it in the overlay and reconciles to the
  server after save. This is why the editors never fight the cursor (see below).
- **A single `source` prop.** `TranslatableField` accepts only `source` (plus presentational props
  like `editorType`, `minHeight`, `dialogTitle`). The old `translation`, `initialContents`,
  `initialStatuses`, and `onSave*` / `onAiTranslate` / `onApprove` / `onMarkAsDraft` props are
  **removed**. Sources are constructed by the **consumer**, never inside the component, so the
  component keeps exactly one input shape.
- **Svelte 5 idioms throughout:** getter-thunks for reactive inputs, `$derived` (never `$effect`)
  for computed state, and the `createX()`-in-`.svelte.ts` factory pattern (the same shape `runed`
  uses).
- **Colocation:** the shared contract, `TranslatableField`, `TranslationEditor`,
  `LanguageStatusBadge`, and `translationUtils` stay under `src/lib/components/Translation/`
  (genuinely reused across routes). `createLearnSource` and `Pages` stay colocated under
  `src/lib/tools/learn/`. Per-consumer construction helpers live at their call site, not in the
  shared tree.

### Why the editors stop fighting the cursor

[RichTextEditor.svelte](../../ui/packages/comhairle/src/lib/components/RichTextEditor/RichTextEditor.svelte)
reconciles its external `value`: a `$effect` calls `editor.commands.setContent(...)` whenever
`value` differs from what the editor last emitted. If `value` ever goes stale relative to the user's
typing, the editor resets and the cursor jumps. Today the dialog avoids this with its local copy;
under this ADR the **optimistic overlay** guarantees `source.contents` reflects the keystroke
immediately, so `value` is never stale, and the local copy is no longer needed.

## Considered options

- **Abstract writes only** (rejected): the component would still fork on reads
  (`translation.textTranslations` vs `initialContents`), so we would delete half the fork and keep
  the other half. No single code path.
- **Keep the component building the source from a `translation` prop** (rejected): the component
  would branch on "did I get `translation` or `source`?", which is the `isTextContentMode` fork
  re-entering through the front door.
- **`Pages` implements `TranslationSource` directly** (rejected): `Pages` is a multi-page controller
  (order, current selection, add / delete / reorder); a source is one field. The `createLearnSource`
  adapter keeps `Pages` single-purpose.
- **Backward-compatible shim keeping the old props** (rejected): two calling conventions plus a
  hidden mapping between them is harder to understand than the status quo, which is the opposite of
  the goal. We migrate all consumers and delete the old props instead.
- **Unify the data model at the backend** (deferred): make every translatable field a `TextContent`
  so there is one representation and the adapter is unnecessary. This is the real cure and remains
  the preferred long-term direction, but it needs a schema migration and data backfill and is out of
  scope for a frontend refactor.

## Consequences

- **`TranslatableField` takes exactly one persistence input: a `source`.** A future reader asking
  "how does this field save?" follows the `source` to a named factory, not an in-component mode
  flag. This ADR is the answer to "what is a `TranslationSource` and why does everything go through
  one?".
- **Persistence logic lives with persistence.** Debounce, save state, and flush are in the sources
  and `translationUtils`; the components render and forward intent. Save indicators are now
  uniformly truthful because `saveState` reflects the real request.
- **The learn / setup step gains a real save indicator and safe page switching** as a consequence of
  using the same contract as every other field, rather than as a special case.
- **The backend data-model split still exists.** The adapter hides it, it does not remove it; the
  learn adapter is the throwaway piece if and when the `TextContent` unification lands.
- **Migration is a single branch of ordered, green commits:** land the contract and both sources,
  migrate the five consumers one at a time, then delete the old props last. `TranslationStatus` and
  `SaveState` are the shared vocabulary, defined once in `translationUtils`.
