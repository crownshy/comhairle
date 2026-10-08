# Permissions Quick Start

## Model and storage

A role assignment links a **User or User group** to a role on one resource.
Each resource module owns its typed `Action`/`Role` policy and two concrete
`UserPermission`/`GroupPermission` models (`sqlx::FromRow`, SeaQuery `enum_def`):

| Module | User table | Group table |
| --- | --- | --- |
| [conversation.rs](conversation.rs) | `conversation_user_permissions` | `conversation_group_permissions` |
| [organization.rs](organization.rs) | `organization_user_permissions` | `organization_group_permissions` |
| [system.rs](system.rs) | `system_user_permissions` | `system_group_permissions` |

[../permissions.rs](../permissions.rs) supplies shared traits, queries, and
authorization. `ResourcePermission` is a query/API projection, not persisted
storage. There is no unified `resource_permissions` table or compatibility view.
System tables do not store a resource ID. Shared API/cache metadata use
`SYSTEM_RESOURCE_ID` (the nil UUID) to identify the global System.

## Authorization

Wire HTTP permission checks through
[../../middleware/permissions.rs](../../middleware/permissions.rs).
Inject `Extension(state.clone())` once around the merged router at the application
root so the extension layer runs before all guards. Guards extract that shared
state; `.with_state(...)` alone does not supply middleware state.
Apply `from_fn_with_state(PermissionRequirement::<Resource>::new(
action), authorize::<Resource>)` as a route layer. Requirements contain only the
action, so router functions do not need application state to configure guards.
For mixed-action routers, layer
each method separately before merging. Extract a caller in the handler only when
its ID is needed for audit data, transaction authorization, or caller-specific behavior.
Dynamic permission routes use `TargetPermissionRequirement` with `authorize_target`.
Keep assignment reauthorization inside the transaction, even with a route guard.
Use `can_perform_action` directly for conditional access and non-HTTP workflows.
`PermissionResource` combines request extraction, resource ID, optional ownership,
and the associated action type. Static authorization accepts only that action type.
Dynamic URL targets dispatch explicitly to a resource module's typed policy.
Discovery uses `roles_for_action` from those same policies; there is no global
`Role` or `Action` catalogue. `PermissionRole::triplet` and `system_triplet` return
recoverable errors for invalid resource/role combinations.
Access is allowed by resource ownership, effective System `super_admin`, or a
matching role from direct grants plus **all current group memberships**.
Grants are additive; there are no explicit denies. Unknown role names do not
grant typed actions.

- Conversation `content_editor`: read/update; `conversation_co_host`: read only.
- Organization `organization_admin`: read/update/delete, membership and permission management.
- System `super_admin`: global authority. Legacy `admin` has no typed actions;
  do not treat it as equivalent to `super_admin`.

## Editing and edge cases

- **Organizations are not recipients.** Resolve `Organization.user_group_id`.
  Membership comes from `user_group_member`, not legacy `User.organization_id`.
- **Direct and inherited roles are independent.** Revoking a direct grant does
  not remove group access. Edit the group grant or membership to remove inheritance.
- **Use transactional workflows.** [assignments.rs](assignments.rs) loads direct
  roles and inherited sources, then atomically replaces only direct roles using
  `expected_version`. A stale save returns Conflict: reload, do not silently retry.
  Retained grants keep their audit metadata; even a no-op save advances the version.
- **Low-level helpers are not authorization boundaries.** `grant_role` and
  `revoke_role` persist changes; callers must enforce access. Assignment saves
  reauthorize inside the transaction. Membership workflows are in
  [../user_group.rs](../user_group.rs).
- **Self-demotion is restricted.** Organization assignment saves reject changes
  to the caller's own direct roles and removal of an inherited administrator role
  through a group they belong to. Membership workflows also forbid self-changes.
- **Administrator continuity is checked at commit.** Every existing Organization
  needs an actual direct or inherited administrator. Once System SuperAdmin is
  bootstrapped, at least one actual User must retain it. An empty group's grant
  does not count. SuperAdmin authority alone does not satisfy Organization continuity.
  PostgreSQL serializes guarded mutations; do not replace this with row counts.
- **Redis is optional, not authoritative.** Cached roles and memberships have a
  five-minute TTL and are checked against PostgreSQL versions. Mutation triggers
  advance versions; invalidate caches after commit. Redis failure falls back to PostgreSQL.
- **Migration rejects unknown legacy resource types.** Resolve those grants before
  applying the split; supported grants retain IDs and audit fields.

From the repository root, run `cargo test -p comhairle_api --lib permissions`
when changing policy, storage, assignment concurrency, or administrator safeguards.