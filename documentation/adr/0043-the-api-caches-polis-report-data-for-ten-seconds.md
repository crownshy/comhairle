# ADR-0043: The API caches Polis report data for ten seconds

**Status:** proposed
**Date:** 2026-09-25
**Branch:** demo branch (the cache itself applies to every consumer of `PolisGetReportData`)

## Context

`PolisGetReportData` builds a report from two Polis calls: `math/pca2` and the comments
list with voting patterns. It had no cache at any layer. The room display polls it every
8 seconds, the admin Insights page and the report builder call it on load, and each call
went straight through to Polis.

That made Polis load linear in the number of screens on a step. One projector was 15
Polis requests a minute. Five booth screens on the same step would be 75, all fetching the
same bytes, because every screen ticks on the same clock and Polis only recomputes its
maths every few seconds anyway.

The pca2 call also sent `lastVoteTimestamp=0`. Polis ignores that parameter. Its
conditional fetch is keyed on `math_tick`, a counter it bumps when the math worker
publishes, and it answers 304 with an empty body when the caller's tick is current. We
never sent one, so Polis always sent the full blob.

The API runs as one replica, so a process-local cache is enough. Redis exists in the
deployment but adds a hop and a serialisation step for no gain at this scale.

## Decision

`PolisClient` keeps two in-memory caches, both `moka`:

- **A report cache**, keyed on poll id, holding the raw pca2 result and comments list
  together, with a 10 second time to live. Misses go through `try_get_with`, so
  concurrent callers for the same poll wait on one upstream fetch instead of each making
  their own. The strict-moderation flag is applied when the report is shaped from the
  cached inputs, so one entry serves both settings.
- **A pca2 memo**, keyed on poll id, holding the last parsed pca2 result and its
  `math_tick`, dropped after an hour idle. A refresh sends that tick; on 304 the memo is
  reused and only the comments list is refetched.

The room display's poller also stops while its tab is hidden and refreshes on return, so
an abandoned browser tab does not keep a poll warm.

## Consequences

- Polis load per active poll is bounded by the cache window, not by screen count: at
  most two upstream calls every ten seconds, and usually one plus a 304.
- Report data can be up to ten seconds behind what Polis has, plus Polis's own math
  recompute lag. Admin Insights inherits that. Nothing on those pages is a live control,
  so nobody is acting on the staleness.
- A Polis failure during a refresh is not cached. The next request retries. The room
  display separately keeps its last good frame on a failed poll, as before.
- The cache is per API process. If the API ever runs more than one replica, each holds
  its own copy and Polis sees one fetch per replica per window, which is still flat in
  screen count. Moving it to Redis is the change if that ever matters.
- `math_tick` is Polis's public conditional-fetch mechanism, but a 304 before any memo
  exists (a conversation with no math result yet) surfaces as an error, exactly as the
  empty-body parse failure did before.
