-- Remove unique constraint from `conversation_id` and `user_id`
-- Add `context` column and add unique constraint between `context` and `user_id`
-- Remove NOT NULL from `conversation_id`

ALTER TABLE bot_service_user_session
    ADD COLUMN context TEXT;

-- Rows with only conversation_id are qa_bot sessions
UPDATE bot_service_user_session
SET context = 'qa_bot'
WHERE context IS NULL
    AND conversation_id IS NOT NULL
    AND workflow_step_id is NULL;

-- Rows with conversation_id and workflow_step_id at the stage are elicitation_bot sessions
UPDATE bot_service_user_session
SET context = 'elicitation_bot'
WHERE context IS NULL
    AND conversation_id IS NOT NULL
    AND workflow_step_id is NOT NULL;

-- Safe-guard any rows missed by the last two blocks
UPDATE bot_service_user_session
SET context = 'unknown'
WHERE context IS NULL;

ALTER TABLE bot_service_user_session
    ALTER COLUMN context SET NOT NULL;

ALTER TABLE bot_service_user_session
    ALTER COLUMN conversation_id DROP NOT NULL;

DROP INDEX bot_session_user_conversation_index;

CREATE UNIQUE INDEX bot_session_user_qa_conversation_idx
ON bot_service_user_session (user_id, conversation_id)
WHERE context = 'qa_bot';

CREATE UNIQUE INDEX bot_session_user_elicitation_workflow_idx
ON bot_service_user_session (user_id, workflow_step_id)
WHERE context = 'elicitation_bot';
