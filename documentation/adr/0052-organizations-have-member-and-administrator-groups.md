# Organizations have member and administrator groups

Each Organization owns two User groups: a members group (`user_group_id`) granted the view-only `organization_member` role, and an administrators group (`admin_group_id`) granted `organization_admin`. Administration is therefore membership of the administrators group, managed like an ACL: anyone with Organization administrator authority adds or removes Users from either group through `PUT`/`DELETE /organizations/{id}/groups/{members|admins}/members/{user_id}`.

Administrators are always also members, so grants made to the members group (for example a Conversation co-host role) keep applying to them. Adding to the administrators group also adds to the members group, demotion only leaves the administrators group, and leaving the members group removes the User from the Organization entirely. We rejected direct per-user `organization_admin` grants because they made the administrator set invisible to group tooling and required a separate role-update endpoint beside the membership one.

Existing safeguards are unchanged: no self-changes, database-enforced administrator continuity ([ADR-0046](0046-organization-administrator-continuity-is-database-enforced.md)), and Super Administrator group guarding. The migration moves existing direct and member-group `organization_admin` grants into the new group.
