# Permissions redesign

## Permission models and storage

Replace the shared resource-permissions table with separate User and User group
assignment tables for each resource type: Conversation, Organization, and System.
This produces six assignment tables for the current resource types. Organization
grants target the Organization's membership group, rather than the Organization
as a separate assignment recipient.

Each resource type has its own model module at
`api/src/models/permissions/<resource_type>.rs`, defining its roles, actions, and
`UserPermission` and `GroupPermission` records. These records derive SQLx
`FromRow` and use SeaQuery `enum_def` for concrete table identifiers.
`api/src/models/permissions.rs` remains the entry point and defines traits for
common permission behavior. The shared `ResourcePermission` is an API/read
projection, not a persisted table. Writes target the six concrete tables, and
cross-resource listings combine their read queries without a compatibility view.

## Administrator safeguards

An existing Organization must retain at least one Administrator. Organization
Administrators may manage other members' Administrator assignments but may not
change their own Organization membership or Administrator role. Membership alone
does not authorize permission changes.

Self-protection also covers indirect changes: an Organization Administrator may
not change a group grant in a way that removes their own inherited Administrator
authority. Enforce this server-side, not merely by disabling direct User editing
in the UI. The last-administrator invariant does not replace this restriction.

At least one actual User must retain direct or inherited Super Administrator
authority. An empty User group holding that role does not satisfy this invariant.
Enforce continuity in the database across grant revocations and membership
removals, including other mutations that could remove the last User's authority.
Concurrent changes must not be able to jointly violate either continuity rule.

Only Super Administrators may grant the Super Administrator role or add Users to
a group holding it. Ownership shortcuts do not override these restrictions.
See [ADR-0046](adr/0046-organization-administrator-continuity-is-database-enforced.md).

## Organization creation

Create the Organization, its membership group, initial administrator memberships,
and initial Administrator assignments in one database transaction. Creation must
not commit an Organization without an Administrator; assignment or membership
failure rolls back creation. Validate administrator continuity against the final
transaction state, allowing the required records to be created together.

Send notification and initial password reset emails only after a successful
commit. Email failures do not roll back Organization creation and may be reported
as notification failures. This supersedes the assignment-failure partial-success
behavior in the Organization creation ADR.

## Organization membership removal

Removing a User from an Organization removes their membership in its membership
group and revokes all direct role assignments to that User on that Organization
in one database transaction. If authorization or an administrator-continuity
constraint rejects the operation, neither membership nor grants are changed.

Preserve the User's direct Conversation and System grants, memberships in other
Organizations, and grants to groups. Roles inherited through the removed
membership cease to apply; roles inherited through other memberships remain.

The transaction advances both the affected membership-set and permission-set
versions. After commit, invalidate both corresponding caches. Removing membership
alone does not change group grants; the direct-role cleanup is a separate part of
this combined Organization removal operation.

## Migration and compatibility

Preserve existing resource behavior and access during migration; the storage
redesign must not silently add, drop, or reinterpret grants. Create one membership
group per existing Organization and migrate its existing members into that group.
Move Organization-recipient grants to the corresponding membership group and
direct User grants to the appropriate resource-specific User assignment table.
Preserve grant audit metadata, including grantor, reason, and timestamp.
After copying these grants, drop the original table. Unsupported legacy resource
types cause migration to fail without discarding their grants; resolve them before
retrying. Do not retain a unified table, archive, or compatibility view.

The newly agreed authorization safeguards remain requirements for the redesigned
system. Every existing Organization has at least one Administrator. Migration
must verify that administrator continuity is preserved without creating new
administrator grants; no remediation for administrator-less Organizations is
expected. Unexpected violations must be surfaced rather than silently changing
access during migration.

Each environment currently runs a single API server instance. Use a coordinated
cutover rather than supporting old and new API versions concurrently; rolling
deployment compatibility and dual writes are not required. Stop the old API
before migrating and start the updated API against the migrated schema, with
the matching UI deployed for the new contract. The exact operational runbook
remains to be defined.

Single-instance deployment does not remove concurrent-request races. Keep the
database-enforced continuity rules, atomic version checks, and version-validated
caches described below.

## Permissions middleware

`api/src/middleware/permissions.rs` defines a single reusable permissions
middleware function. Protected routes explicitly declare a typed resource-specific
action and a target-resource resolver through the shared traits. Permissions are
not inferred from URL conventions or HTTP verbs.

For example, a Conversation update route declares its Conversation update action
and resolves the target Conversation from its path ID. The common middleware
handles authentication and applies the shared authorization rules: ownership
short-circuits role evaluation, Super Administrator authority allows globally,
and otherwise direct and inherited group grants are evaluated additively.
Mandatory Super Administrator restrictions and administrator-continuity
invariants still apply to assignment and membership mutations.

## Permission and membership caching

Group membership is checked separately from permissions. Keep membership and
permission caches separate rather than caching a combined inherited-access
decision. Membership determines which groups' grants are considered; a group's
permission entries do not imply that a User is currently a member.

Granting or revoking a permission invalidates the relevant permission cache
entries. For transactional assignment edits, invalidation follows a successful
commit and covers every changed assignment, including User group grants and
Super Administrator grants. Failed or conflicting saves do not change grants.

Adding or removing a group member invalidates the relevant membership cache
entries after a successful commit. Membership changes do not alter the group's
grants and do not require invalidating its permission entries.

Both membership and permission cache entries have a five-minute TTL (300 seconds)
so entries expire rather than remaining indefinitely. Expiration is a backstop,
not a substitute for invalidation on changes.

Use database-backed versions to validate both caches. Every permission or
membership mutation advances the affected set's version in the same transaction
as the data change. Versions must not be reused when a set becomes empty or is
recreated.

Each cached value includes its version. Permission checks read the authoritative
versions from Postgres and use cached data only when its version matches. On a
miss or mismatch, load the data and its version from the same database snapshot
before caching them. Never attach a newer version to data read before that version
was committed.

Cache deletion remains an optimization: failed deletion or an older request
repopulating an entry cannot make a stale version valid. Redis failures fall back
to Postgres; inability to validate against Postgres must not permit access based
on unvalidated cached data. This requires database version lookups on permission
checks, while still caching membership lists and role assignments.

See [ADR-0051](adr/0051-permission-caches-use-database-backed-versions.md).

## Assignment editing

Role assignment changes target the assignment recipient: a User or a User group.
A group assignment is edited on the group, not by promoting or demoting individual
members who inherit it. Editing a direct User assignment does not change inherited
access.

Recipients may hold multiple roles on the same resource. Each selected role is a
separate assignment, and allowed actions are the union of direct and inherited
roles. Unchecking one role revokes only that assignment on the selected recipient;
other assignments and other sources of inherited access remain unchanged.

Save Changes applies all requested grants and revocations for the selected
recipient and target resource in one database transaction. If authorization,
validation, or a database constraint rejects any change, none of the requested
changes are applied. The editor must not implement a save as independent
grant/revoke requests that can partially succeed.

If the selected recipient's assignments on the target resource have changed since
they were loaded, reject the save with an HTTP `409 Conflict` and apply no changes.
The stale-state check must be concurrency-safe within the save transaction, not a
separate preflight check. The editor reloads current grants for review rather than
silently overwriting another administrator's changes or retrying automatically.
Reuse the database-backed permission-set version used for cache validation,
scoped to the selected recipient and target resource. Load grants and their
version from the same database snapshot and submit that version with the save.
The transaction atomically verifies the expected version, applies the grant
changes, and advances the version; two saves against the same version cannot
both succeed. No separate editor revision mechanism is needed.

## Reusable assignment editor

The supplied UI reference establishes the direction: a searchable recipient table,
recipient-type and role filters, current-role badges, and an Edit Roles action
opening a side panel with role checkboxes and explicit Save Changes and Cancel
actions. Adapt this pattern to Comhairle's existing components and visual language,
rather than adopting the reference application's branding or example roles.

Recipient discovery is authorization-scoped. Search and listing endpoints expose
only Users, Organizations, and User groups the caller is authorized to discover,
not the entire platform directory. Enforce discovery authorization server-side;
the editor's filters are not an access-control boundary. Apply the same visibility
rules when displaying inherited-role source Organizations and User groups.

Organization recipients represent their Organization membership groups. Group
Super Administrator assignments remain supported without a UI for assigning them.

The editor receives a target resource consisting of its resource type and resource
ID. It uses the type to load the appropriate role catalogue and the type plus ID
to load existing grants for that individual resource. The containing page supplies
the target; the editor does not act as a global resource browser.

When editing a User's direct grants, show inherited roles read-only in a separate
section, identifying their source Organizations or User groups. Editable role
checkboxes represent only direct assignments to the selected User. Revoking a
direct assignment must not imply that inherited access has also been removed;
group assignments are edited on the group, not on an inheriting User.

## Translation authorization redesign

The following decisions describe the intended replacement for the current
System Translate checks. They are not yet implemented.

### Scope

Associate scope with `text_content`, shared by every locale's
`text_translation` row. Each Text content belongs to exactly one Conversation,
one Organization, or the System. Step text belongs to its Conversation.

Use nullable `conversation_id` and `organization_id` fields. At most one may be
populated; both null means System content, not an unassigned state. Migration
must establish the scope of every existing record and prove that records left
with both IDs null genuinely belong to the System. Unresolved records must not
silently become System content.

The server assigns scope at creation; scope is immutable afterwards. Validate
scope whenever a resource accepts a Text content reference, including nested
tool configuration and imports. Reject cross-scope references. Reusing content
in a different scope creates a new copy rather than sharing or reassigning the
existing record.

Deleting the scope-owning Conversation or Organization atomically deletes its
Text content and every locale version. Never clear a scope foreign key on
deletion: both IDs null would incorrectly reclassify the content as System.
Deleting an individual Step must not delete unrelated Conversation-scoped text.

### Roles and capabilities

Remove the System Translator role. Retain the Conversation Translator role and
introduce an Organization Translator role. A System Administrator's translation
authority applies only to System-scoped content, not all resources. Super
Administrators retain the existing global override.

Resource translation authority permits editing, generating, approving, and
deleting non-primary locale versions. Editing primary-locale text or its
configuration requires the resource's update authority. Resource Administrators
and owners retain both capabilities.

| Resource authority | Admin translation reads | Primary text and configuration | Non-primary locale versions |
| --- | --- | --- | --- |
| Read | Allowed | Denied | Denied |
| Update | Allowed | Allowed | Allowed |
| Translate | Allowed | Denied | Allowed |
| Administrator or owner | Allowed | Allowed | Allowed |

Update authority includes creating source Text content and deleting whole
Text content records. Translate authority includes generating, approving, and
deleting non-primary locale versions. Organization Translator includes
Organization read authority, matching Conversation Translator's existing read
capability. Grants remain additive. Existing participant-facing localized reads
retain their current visibility rules.

Rich-text AI translation operates through the original scoped Text content.
Authorize that operation against the original content, translate without a
separately client-created temporary Text content record, and save only the
target locale. Do not grant source creation or deletion authority to Translators
to preserve the current temporary-record workflow.

### Cutover

Revoke all existing System Translator assignments, both direct User grants and
User group grants, at cutover. Do not generate replacement resource grants.
Administrators manually grant the required Conversation or Organization roles
afterwards. This intentionally removes those recipients' previous global
translation authority and is an exception to the grant-preservation rule for
the earlier permissions-storage redesign.

### Implementation coverage

Inventory existing Text content references before writing the backfill, including
Conversation fields, Steps and nested tool configuration, proposals and their
sections, Events, Reports, Organizations, and Regions. Classify genuinely
System-owned text explicitly during that inventory. Detect missing, ambiguous,
and cross-scope references rather than treating an unmatched record as System.

Wire scope assignment and reference validation through every creation path,
including copies, imports, and generated text. Enforce the mutually exclusive
resource IDs in the database, and create resources and their scoped source text
atomically where required.

Resolve authorization from the stored Text content scope on every admin
translation route, not from a resource ID supplied alongside the edit. Apply the
capability matrix to reads, source creation and deletion, configuration updates,
locale upserts and updates, approval changes, locale deletion, and single-locale
and bulk AI generation.

Determine source-versus-target authority from the authoritative primary locale.
When an update changes a locale, check both its existing and proposed locale:
renaming a non-primary translation to the primary locale must not bypass source
editing authority. Keep authorization-sensitive checks consistent with the
mutation when concurrent configuration changes occur.

Update role catalogues, assignment editors, generated API contracts, and
translation UI controls to reflect the new scopes and separate capabilities.
Server-side enforcement remains mandatory even when controls are hidden.

Acceptance tests must cover direct and inherited roles, cross-resource denial,
System Administrator versus Super Administrator access, primary-locale write
denial for Translators, the agreed read matrix, rich-text AI translation without
temporary Text content, cross-scope reference rejection, migration
classification, removal of legacy System Translator grants, and atomic scoped
resource deletion without reclassification as System.
