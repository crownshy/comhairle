-- Backs up the API's reason checks in the database. Both constraints are checked at commit,
-- so a save that swaps two reasons' positions or labels doesn't trip them partway through.
-- Labels use an exclusion constraint because a unique index on lower(label) can't be
-- deferred.

ALTER TABLE moderation_policy_reason
    ADD CONSTRAINT moderation_policy_reason_position_unique
    UNIQUE (moderation_policy_id, position) DEFERRABLE INITIALLY DEFERRED;

ALTER TABLE moderation_policy_reason
    ADD CONSTRAINT moderation_policy_reason_label_unique
    EXCLUDE USING btree (moderation_policy_id WITH =, (lower(label)) WITH =)
    DEFERRABLE INITIALLY DEFERRED;
