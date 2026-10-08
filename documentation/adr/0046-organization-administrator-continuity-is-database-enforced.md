# Organization administrator continuity is enforced by the database

An existing Organization must retain at least one Administrator. Prohibiting administrators from changing their own membership or role is insufficient: two administrators could concurrently demote or remove each other. Enforce administrator continuity in the database, not solely through application checks, so conflicting changes cannot both succeed.

The invariant spans role assignments and membership, so a plain row-level PostgreSQL `CHECK` constraint cannot express it. Database enforcement must serialize conflicting changes and validate the resulting administrator availability; an unsynchronized count in a trigger is insufficient. The concrete enforcement mechanism remains to be selected alongside the permissions schema.

The same database-enforced continuity requirement applies to Super Administrator authority across the platform: at least one actual User must retain direct or inherited authority. An empty group grant does not count. Enforcement must cover grant revocations, membership removals, and any other mutations that could remove the last User's authority, including concurrent changes.

Organization Administrator self-protection applies separately from continuity: an administrator cannot remove their own inherited Administrator authority by changing a group grant, even if another administrator would remain. Enforce this for indirect group edits as well as direct User edits.