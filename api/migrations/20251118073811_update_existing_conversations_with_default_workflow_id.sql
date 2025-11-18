-- Add migration script here
-- Update existing conversations to have default_workflow_id set to their first workflow

UPDATE conversation 
SET default_workflow_id = (
    SELECT w.id 
    FROM workflow w 
    WHERE w.conversation_id = conversation.id 
    ORDER BY w.created_at ASC 
    LIMIT 1
)
WHERE default_workflow_id IS NULL;