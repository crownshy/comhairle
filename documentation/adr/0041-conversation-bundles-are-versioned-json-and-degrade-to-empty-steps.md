# ADR-0041: Conversation bundles are JSON with a hand-bumped version, and Steps that can't be imported arrive empty

**Status:** Proposed - to be discussed with the team
**Date:** 2026-09-17

## Context

We want to copy a Conversation from one environment to another (staging to prod and back)
as a Conversation bundle: a JSON file holding its configuration and no participant data.
The environments run different builds most of the time. A Step's tool config also points
at things that only exist on one environment: HeyForm `survey_id` / `workspace_id` /
`project_id`, Polis `poll_id`, `server_url`, and per-tool admin credentials. So import
cannot copy configs as they are. It has to set every tool up again and replay its content.

Nothing in the app records a version today. There is no API version, app version or git
SHA at runtime. The only marker is the newest migration name, and that differs between
staging and prod almost every day even when nothing an export touches has changed.

## Decision

- **Import always creates a new draft Conversation.** It never overwrites or merges into
  an existing one. The importer becomes owner.
- **The bundle is one JSON file.** The data is small, so there is no zip. The cover image
  travels as its public URL and is downloaded and re-uploaded on import.
- **Two hand-bumped versions decide compatibility.** `bundle_format_version` covers the
  comhairle part of the bundle. Each Step carries a `tool_payload_version` for the part
  owned by its tool (for HeyForm, the fork's own format version; see ADR-0042). Source
  environment and export date are recorded for people to read and are never compared.
- **The bundle is parsed strictly against the target's Rust types.** A newer
  `bundle_format_version` or a parse failure rejects the import with a message naming the
  version or the field path. Nothing is created.
- **A Step whose payload can't be read or whose tool setup fails becomes an empty Step**
  with fresh default setup. The Conversation is still created, and the import response
  lists those Steps and why so the admin knows what to redo.
- **Import runs inside the request**, like creating a Conversation from a template.

## Considered options

- **Comparing migration name or git SHA.** Rejected: nearly every import would warn, and
  admins would learn to ignore the warning.
- **A zip with media files, an import preview, a background job and a stored import
  report.** Deferred: more than an MVP needs. The preview and stored report can be added
  later without changing the bundle.
- **Blocking the whole import when any Step fails, or rolling back.** Rejected: one flaky
  Polis call would throw away the whole import, and cleaning up HeyForm and Polis can fail
  too.
- **Importing into an existing Conversation.** Rejected: matching Steps and handling
  existing participant data is where drift actually breaks things.

## Consequences

- Anyone changing a Step config type or the bundle shape has to bump the matching version.
  Forgetting is caught by strict parsing, but as a rejected import.
- The list of empty Steps only exists in the import response. If the admin closes the page
  they lose it; a stored report is the follow-up if that turns out to matter.
- Copying knowledge base documents would need a background worker (new dataset, re-upload,
  re-parse). Two Conversations sharing one `knowledge_base_id` was ruled out, because
  documents added to one would show up in the other and deleting from one would remove
  from both.
