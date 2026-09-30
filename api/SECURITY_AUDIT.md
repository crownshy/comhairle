# Security Audit — Comhairle Rust API (`api/`)

Scope: axum / SQLX / sea-query / Postgres API server. Read-only review. Router/middleware
mapped first, then per-handler authz, SQLi, secrets, JWT/session, input validation, logging,
CORS, crypto, deps, rate limiting.

**Architecture note (load-bearing):** There is **no route-level auth middleware**. The only
global layer is request logging + CORS + body limit (`api/src/lib.rs:309-315`). Every handler
must self-protect via an extractor (`RequiredUser` / `RequiredAdminUser` / `RequiredRole` /
`authorize()`) or by scoping queries to `user.id`. Where that is forgotten, the endpoint is
fully open. `RequiredRole` only validates the path `conversation_id` (`models/permissions.rs:264-266`),
NOT sibling `workflow_id`/`event_id` path params.

---

## Critical

### C1. `update_report` — mutation with zero authentication
`api/src/routes/reports.rs:76` (PUT `/conversation/{conversation_id}/report/`, mounted `lib.rs:246`)
- **Class:** Broken access control (missing auth).
- **Why:** Signature is `State`, `Path`, `Json` — **no user extractor**. Any anonymous caller
  can overwrite the published Report of any conversation by UUID via `models::report::update`.
  Sibling `create_report` correctly uses `RequiredAdminUser`; only `update` was missed.
- **Fix:** Add `RequiredAdminUser` or `RequiredRole<Conversation,(Owner,(Contributor,))>`.


### C2. `get_participation_report` — unauthenticated demographic PII
`api/src/routes/workflows.rs:219` (GET `.../workflow/{workflow_id}/participation_report`, registered `:363`)
- **Class:** Broken access control / sensitive data exposure.
- **Why:** Only extractors are `State` + `WorkflowPathCtx`, which merely parses the `workflow_id`
  UUID (`workflows.rs:95-111`) with **no auth**. Any anonymous caller who knows/guesses a workflow
  UUID gets `user_profile::get_demographic_report` — aggregated citizen demographics — for any
  workflow in any conversation.
- **Fix:** Require conversation role / `authorize` / at minimum `RequiredAdminUser`.
- **Stuart Note** these should be aggregated and they are meant to be used as part of the public report so I think this is ok.
  I do think we should only return this after say 15 people have taken part though so we avoid small number identififcation.

### C3. Hardcoded default JWT signing secret baked into the binary
`api/src/config.rs:15-18` — `.set_default("jwt_secret", "ababa039cc54b5df83e8899c3c5839e096379d507263c732eb54c52477bf8087")`
- **Class:** Secrets / JWT.
- **Why:** Fallback signing key for **all** JWTs (session cookies, password-reset, email-verify,
  OTP links). If `jwt_secret` is ever unset, the server silently signs with this public,
  source-committed value → anyone forges a session JWT for any `id` (`auth.rs:246`, validated at
  `auth.rs:1032-1055`) and impersonates any user incl. admins. No startup assertion it was overridden.
- **Fix:** Remove the default. Make `jwt_secret` required; fail fast at startup if absent or equal
  to the known default. Rotate in every deployment. **(Note: this same secret appears committed in
  `helm_charts/comhairle/templates/comhairle_secrets.yml:11` — see the helm audit; rotate together.)**

### C4. Full config (all secrets) logged at startup
`api/src/lib.rs:323` — `tracing::info!("Running with config {:#?}", state.config);`
- **Class:** Sensitive data exposure.
- **Why:** `ComhairleConfig` (`config.rs:61-84`) holds `jwt_secret`, `mailer.password`, translator
  `api_key`, bot-service `api_key`, categorization `api_key`, `database_url` (with DB password).
  `Debug`-derived → printed cleartext on every boot to stdout/deploy logs. Same class as the known
  sqlx hash leak.
- **Fix:** Don't log the whole config. Redact secret fields (custom `Debug` / `secrecy::Secret` /
  log only non-secret keys).

---

## High

### H1. Default tracing level is `debug` with `sqlx=debug` — query params logged
`api/src/main.rs:30-46` — fallback `EnvFilter` = `"debug,sqlx=debug,tower_http=info,axum::rejection=trace"`
- **Class:** Sensitive data exposure (logging). **This is the root cause of the password hashes
  found in `helm_charts/deploy_pod*.log`.**
- **Why:** When `RUST_LOG` is unset, sqlx logs at DEBUG → SQL with bound parameter values (password
  hashes, OTP codes, reset tokens, emails, IPs). Fallback should be safe-by-default.
- **Fix:** Default to `info,sqlx=warn`. Never ship a debug default. Confirm deployments set `RUST_LOG` explicitly.

### H2. `get_workflow_stats` — unauthenticated stats enumeration
`api/src/routes/workflows.rs:211` (GET `.../workflow/{workflow_id}/stats`, registered `:354`)
- **Class:** Broken access control.
- **Why:** Same as C2 — `State` + `WorkflowPathCtx` only, no auth. Anonymous enumeration of
  participation stats for any workflow by UUID.
- **Fix:** Gate with conversation role / `authorize` / `RequiredAdminUser`.
- **Stuart Note** This being public is intentional I think as it will inform a public report 

### H3. IDOR on event attendances (read single + list roster)
`api/src/routes/event_attendances.rs:57` (`get`), `:37` (`list`)
- **Class:** IDOR / broken access control.
- **Why:** Both use `RequiredUser` but **discard the user** (`_user`) and never scope the query or
  call `authorize`. `get` loads any attendance row by UUID (who attends which event + role) across
  conversations; `list` dumps the full attendee roster of any event.
- **Fix:** `RequiredRole<Conversation,…>` / `authorize`, scope the query to the path event, or (for
  `get`) restrict to the attendance's own `user_id`.

### H4. Auto-created org-admin accounts use a predictable, brute-forceable password
`api/src/models/users.rs:~290` — `organization_admin_temporary_password()` → `format!("TempAdmin#{}Aa1", gen_id())`,
used by `create_organization_admin_user` which grants the system `Admin` role.
- **Class:** Crypto / weak credential.
- **Why:** `gen_id()` (`tools/id.rs`) is 6 chars of Crockford base32 from **non-cryptographic**
  `rand::thread_rng()` with a blocklist shrinking the space. Password is `TempAdmin#XXXXXXAa1` — only
  6 weak chars unknown, on an account that already holds admin. Login rate-limit (5/min/IP) slows but
  doesn't stop a distributed attack.
- **Fix:** Generate from CSPRNG (`OsRng`) ≥128 bits; force reset on first login; expire it.

### H5. Mass-assignment of `organization_id` / `email_verified` on self-service update
`api/src/routes/user.rs:285` `update_user_details` (`RequiredUser`) → `models::users::update_user`
(`api/src/models/users.rs:561-607`) writes `organization_id` and `email_verified` verbatim to the
caller's own row.
- **Class:** Privilege escalation (mass assignment).
- **Severity note:** The authz sub-agent rated this **Critical** — because
  `can_perform_resource_action` OR-includes the user's `organization_id` roles
  (`permissions.rs:918-927`), so a user setting `organization_id` to a victim org inherits that org's
  roles, and can self-set `email_verified=true`. The consolidating pass downgraded to **needs
  confirmation**, noting the handler is otherwise self-scoped (operates only on `user.id`, no client
  user-id). **Action: confirm whether org membership alone grants any privilege. If yes → Critical.**
- **Fix:** Remove `organization_id` and `email_verified` from the self-service `UpdateUserRequest`
  path, or gate them behind `RequiredAdminUser`.

---

## Medium

### M1. Weak / non-CSPRNG OTP codes, delivered inside a URL
- Codes: `otp::create` (`api/src/models/otp.rs:98`) sets `code = gen_id()` — 6 Crockford-base32 chars
  from `thread_rng` (not CSPRNG) with entropy-reducing blocklist. Used as a login credential in
  `login_otp`/`login_otp_token` (`auth.rs:481,561`).
- Delivery: `create_otp` (`auth.rs:530-546`) embeds the plaintext OTP into a signed JWT placed in the
  login URL → leaks via history/referrer/proxy/logs (H1).
- **Class:** Crypto (weak randomness) + sensitive data exposure.
- **Fix:** Generate OTPs from `OsRng`; per-account rate-limit + lockout on accept; keep short single-use links.

### M2. Verbose DB error strings returned to clients
`api/src/error.rs:34-35, 419-423` — `DatabaseError(sqlx::Error)` / `DbError(String)` fall into the
`_ => 500` arm and body is `{"err": self.to_string()}` → raw sqlx/Postgres text (table/column/constraint
names) returned to caller. Aids schema mapping / SQLi probing.
- **Fix:** Generic client message for internal/DB variants; log detail server-side only.

### M3. No account-scoped throttling / lockout; XFF-spoofable IP limiter
`api/src/middleware/rate_limit.rs` (applied only to `/auth` router, `lib.rs:195-199`)
- **Why:** Limiter is **IP-based only** (`SmartIpKeyExtractor`) and trusts `X-Forwarded-For`/`X-Real-IP`
  (`request_logging.rs:84-101`). If reachable without a trusted proxy stripping them, a client rotates
  the key and bypasses the limit. No per-account failed-attempt lockout on `login`/`login_otp`. Other
  sensitive routers (permission grants, admin mutations) have no limiter.
- **Fix:** Honor `X-Forwarded-For` only from a trusted proxy hop; add per-account lockout/backoff.

### M4. Login/reset/OTP user-enumeration via distinct responses
`auth.rs:401` (`login` → 404 `NoUserFound` for unknown email vs 401 `WrongPassword`), `password_reset_create:663`,
`create_otp:517` (404 for unknown emails). `login_annon` is correctly hardened (`auth.rs:447-450`); email flows aren't.
- **Fix:** Uniform response/timing for unknown vs wrong-password; `password_reset_create` always 204.

---

## Low / Informational

- **L1. Test-only routes in prod.** `/auth/test_requires_roles/{conversation_id}`, `/auth/test_api_key_extraction`
  (`auth.rs:1268-1284`) registered unconditionally (code comments say to remove). Gate behind `#[cfg(test)]`/debug flag.
- **L2. Hardcoded admin backdoor regex.** `is_user_admin` grants admin to any email matching
  `^test(?:[1-9]|10)@crown-shy\.com$` (`auth.rs:42-47`). If registerable in prod → privilege escalation by signup. Remove for prod.
- **L3. Session cookie `SameSite=None`.** `Secure + HttpOnly + SameSite=None` (`auth.rs:428-433,463-468,1128-1134`).
  HttpOnly/Secure good; SameSite=None (needed for cross-origin SPA) widens CSRF surface for cookie-authed state
  changes. Ensure CSRF defenses / prefer bearer-token path.
- **L4. `decode_jwt` collapses all errors to 500** (`auth.rs:718-724`) — token still rejected (expiry/signature
  validated by `Validation::default()`), so observability not bypass. `validate_jwt` also `unwrap()`s
  `Uuid::parse_str` (`auth.rs:1047`) → malformed-but-valid-signature id panics that request (DoS-adjacent, low).
- **L5. `admin_users` config auto-grants admin on signup** (`auth.rs:328-341`). By design; keep the list tight and emails verified.

---

## Verified NOT vulnerable (false-positive control)

- **SQL injection: none.** Full pass over every `format!`-near-SQL, `Expr::cust`, raw `query`/`query_as`,
  LIKE/ILIKE, dynamic sort/order. All user input bound as parameters via `build_sqlx` or typed `*Iden`/`Order`
  enums. LIKE uses `format!("%{v}%")` for the *bound value*, not SQL text. The one `format!`-built query
  (`models/user_conversation_preferences.rs:328`) concatenates constant literals and binds `conversation_id`.
- **CORS not wildcard.** `lib.rs:154-190` explicit allow-list with `allow_credentials(true)`. Ensure prod origins
  come only from vetted config (`whitelisted_domains`), not the hardcoded localhost/stage entries.
- **API keys:** SHA-256 hashed, `OsRng` 32 bytes, revoked/expiry checks, never traced (`models/api_key.rs`);
  creation admin-only, self-minted (`routes/api_keys.rs`).
- **Password hashing:** Argon2 default params + per-hash `OsRng` salt (`auth.rs:178-186`); strength enforced
  (16+ chars, zxcvbn ≥3) on signup and reset.
- **Permissions routes** (`grant`/`revoke`/`list`) all call `authorize(...)` against the path resource — no escalation.
- **`resources.rs` (`get_resource`, `upload_request`) is DEAD CODE.** Contains zero-auth handlers but is not
  declared as a module (absent from `routes.rs`), not mounted, references a non-existent `state.s3_client` — does
  not compile into the server. **Recommend deleting the file** so it can't be wired up later.

---

## Priority fix order
1. **C1** `update_report` zero-auth PUT, **C2** unauth demographic report — add auth now.
2. **C3** remove default JWT secret + fail-fast, **C4** stop logging config, **H1** fix default log level
   (all three prevent secret/token compromise; H1 = root cause of the leaked hashes in deploy logs).
3. **H2** unauth stats, **H3** attendance IDOR, **H5** mass-assignment (confirm severity).
4. **H4/M1** CSPRNG for temp admin passwords + OTPs; **M3** per-account throttling.
5. **M2** generic DB errors; **M4** enumeration; **L1/L2** remove test routes + admin backdoor from prod.

**Dependencies:** versions current (argon2 0.5, jsonwebtoken 9.3, recent sqlx/axum). `cargo audit` not run this
pass — add to CI. No specific vulnerable crate identified.
