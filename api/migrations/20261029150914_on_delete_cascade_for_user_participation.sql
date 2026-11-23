-- Cascade conversation deletes through to user_participation rows.
--
-- The previous migration (20261015132950) cascaded workflow_conversation_id_fkey
-- but missed user_participation_workflow_id_fkey, so deleting a conversation still
-- fails when participation rows exist.
--
-- Note: workflow_step_workflow_id_fkey is intentionally left as NO ACTION --
-- workflow steps hold external tool data that must be cleaned up before a
-- conversation can be deleted.
ALTER TABLE user_participation
    DROP CONSTRAINT user_participation_workflow_id_fkey;

ALTER TABLE user_participation
    ADD CONSTRAINT user_participation_workflow_id_fkey
        FOREIGN KEY (workflow_id)
        REFERENCES workflow (id)
        ON DELETE CASCADE;
