# ADR-0006: The Configure page is fully autosave (no explicit Save button)

**Status:** accepted
**Date:** 2026-07-17

## Context

The conversation admin **Configure** page
([configure/+page.svelte](../../ui/packages/comhairle/src/routes/(admin)/admin/conversations/[conversation_id]/configure/+page.svelte))
was one long `superForm` with a single **Save Changes** button at the bottom. We split it into
sub-tabs (Details / Content / Access / Team) so switching between groups of settings doesn't mean
scrolling a long page. That raised the question of where the Save button should go now that its
fields are spread across tabs.

Auditing what actually persisted how made the answer obvious: **almost everything on the page
already autosaved, and the Save button only uniquely mattered for six toggles.**

- Every `TranslatableField` (Title, Short description, Description, and the rich fields: Privacy
  policy, Short privacy policy, FAQs, Thank-you message, Call to action) autosaves on a 1s debounce
  straight to the API once it is `textContent`-backed
  ([TranslatableField.svelte](../../ui/packages/comhairle/src/lib/components/Translation/TranslatableField.svelte)).
  Title / Short description / Description are always `textContent`-backed (`Translation3` in the
  api-client schema); the nullable rich fields create-then-autosave on first edit.
- **Language options** and the **Banner image** already saved immediately through their own handlers.
- Only the six **Access** toggles (`isPublic`, `isInviteOnly`, `autoLogin`, `enableQaChatBot`,
  `enableSignupPrompts`, `showThankYouPageAnnonInstructions`) relied solely on the Save button.

So the single "Save Changes" button was actively misleading: it read as "save the whole page" while
~90% of the page had already saved itself seconds earlier, and clicking it merely re-sent those
fields redundantly (via a different snake-cased `UpdateConversation` path).

## Decision

**Make the whole Configure page uniformly autosave and remove the Save button.**

- The six toggles gain per-change autosave, mirroring the existing Banner / Language handlers: five
  go through `UpdateConversation({ [field]: value })` and `autoLogin` through
  `UpdateConversationWorkflow({ auto_login })`, each wrapped in `tryCatchAsync`, followed by
  `invalidate('conversation:meta')` and a "Setting updated" toast. On failure the optimistic switch
  state is reverted and an error toast is shown.
- The `updateConversation` submit handler, the `<form>` submit, and the Save button are removed.
  `superForm` is kept only for field binding and inline validation display, not for submission.

The `UpdateConversation` route already accepts partial updates (the page had long called it with
tiny payloads like `{ image }` and `{ primary_locale, supported_languages }`), so no backend or
api-client change was needed — this is purely a client-side save-model change.

## Considered options

- **Keep one global Save button** (rejected): least change, but preserves the deception — the button
  only ever mattered for the toggles while everything else autosaved.
- **Per-tab Save buttons** (rejected): only the Access tab has any button-only fields, so this
  produces a lone, asymmetric Save on one tab out of four.
- **Full autosave, no button** (chosen): uniform behaviour, removes the misleading affordance, and
  makes tab-switching inherently safe — there is never a pending edit to lose.

## Consequences

- **No explicit Save affordance on an admin form.** A future reader may expect one; this ADR is the
  answer to "why is there no Save button here?". Autosave is now the page's contract.
- **Toggle feedback is a toast for now; an inline per-toggle "Saving → Saved" indicator is the
  intended follow-up.** English-only fields show no autosave badge (the `TranslatableField` badge
  only renders when other languages exist), so toggles use a toast as their confirmation. The
  preferred end state is a quiet inline indicator per toggle row (like `TranslatableField`'s), which
  is left as a `// TODO` at the toggle handler.
- **A toast fires per toggle change.** Flipping several switches in quick succession stacks toasts;
  acceptable on this page, and superseded once the inline indicator lands.
