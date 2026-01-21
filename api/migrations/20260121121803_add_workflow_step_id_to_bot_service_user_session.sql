-- Adds workflow_step_id to bot_service_user_session table
-- Allows sessions to be created for conversation steps like elicitation bots

ALTER TABLE bot_service_user_session 
    ADD workflow_step_id UUID DEFAULT NULL,
    ADD CONSTRAINT bot_session_workflow_step_id_fk
        FOREIGN KEY (workflow_step_id)
        REFERENCES workflow_step (id)
        ON DELETE CASCADE;
