# ADR-0070: Unscopable translations are preserved for manual review

**Status:** accepted
**Date:** 2026-11-13

## Context

TextContent created before resource deletion cascades may outlive its original resource.
The translation scope backfill cannot determine an owner for these rows. Resource writes
can also encounter text whose Conversation or Organization scope cannot be resolved.
Neither case should discard translations or classify them as System content.

## Decision

TextContent has a persistent `is_orphaned` flag. The scope migration flags content with
no verifiable resource reference, or a reference whose resource scope cannot be resolved.
At runtime, the resource-reference trigger flags direct and JSON-referenced content when
a non-System scope has no resource ID, instead of rejecting the write for that reason.
Existing scope IDs are retained.

Missing TextContent references, conflicting backfill scopes, and known scope mismatches
remain errors. Flagged content cannot be attached to a verified scope until manually
reviewed. The flag is not automatically cleared.

`GET /translations/orphaned` returns an array of the existing `TranslationDto` shape:
`textContent` metadata and `textTranslations` containing every locale. Only Super
Administrators may list or access flagged content through the translations API, including
those who inherit the role through a User group. Null scope IDs on flagged content do not
grant System translation access.

## Consequences

Translations remain available for manual investigation without blocking scope migration
or the affected resource write. This is not general garbage collection: removing a
reference does not automatically flag its old content. Recovery or deletion is a manual
operation; this change adds no repair endpoint.
