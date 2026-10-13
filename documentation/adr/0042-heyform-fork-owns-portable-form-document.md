# ADR-0042: The HeyForm fork owns a versioned, portable form document

**Status:** Proposed - to be discussed with the team
**Date:** 2026-09-17

## Context

A HeyForm Step's content lives in HeyForm's own database, not comhairle's: fields, hidden
fields, logic, variables, translations, settings and theme. Moving a HeyForm Step between
environments (ADR-0041) means moving that form between two HeyForm servers.

The fork has no way to do that. `duplicateForm` copies within one server. The write API
is split across `updateFormSchemas` (fields only), `updateFormLogics`,
`updateFormVariables`, `updateFormTheme` and per-field hidden field mutations, and
translations have no write path. Our SDK's `formDetail` reads only fields, settings and
theme, which is why launch today carries over fields and nothing else.

Logic rules reference fields by id, so a copy that generates new field ids silently
breaks logic.

## Decision

Add three endpoints to the fork and treat the form document as opaque in comhairle:

- `exportForm(formId)` returns a document with fields, hiddenFields, logics, variables,
  translations, settings and themeSettings, plus a `formatVersion` the fork bumps when
  that shape changes.
- `validateFormImport(document)` says whether this server can import the document, and
  why not.
- `importForm(projectId, document)` creates a form from it, keeping field ids.

comhairle stores the exported document in the bundle as-is, uses `formatVersion` as the
HeyForm Step's `tool_payload_version`, and never parses the document itself. After import
it makes sure the `comhairle_user_id` hidden field exists. Submissions, integrations and
analytics are not part of the document. Images uploaded inside HeyForm stay as URLs on the
source server for now.

## Considered options

- **comhairle stitches the form together from the existing mutations.** No fork change,
  but comhairle would have to understand HeyForm's internal schema, it takes about six
  calls with no transaction, and translations and some settings can't be written at all.

## Consequences

- This is a paired change. The fork has to be deployed to every environment before the
  comhairle side works there, and the fork's CI pin step is broken, so `helm_charts` is
  pinned by hand.
- A target whose fork is older than the source's (no `validateFormImport`, or a lower
  `formatVersion`) gets an empty HeyForm Step with a warning, not a broken form.
- The launch copy (`api/src/tools/heyform.rs`) could move onto `exportForm` /
  `importForm` too, which would fix launch dropping logic and theme. Not decided here.
