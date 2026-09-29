-- Make user_id nullable (synced statements may have no comhairle user) and
-- add a unique constraint so sync can upsert by (workflow_step_id, polis_statement_id).

ALTER TABLE polis_statement_aux
    ALTER COLUMN user_id DROP NOT NULL;

ALTER TABLE polis_statement_aux
    ADD CONSTRAINT polis_statement_aux_workflow_statement_key
    UNIQUE (workflow_step_id, polis_statement_id);
