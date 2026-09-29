-- Add migration script here
-- Backfill user_conversation_preferences for existing user participations
-- This creates default preferences (all notifications disabled) for users who have participated in workflows

INSERT INTO user_conversation_preferences (user_id, conversation_id, receive_updates_by_notification, receive_updates_by_email, receive_similar_conversation_updates_by_email, receive_similar_conversation_updates_by_notification)
SELECT DISTINCT 
    up.user_id,
    w.conversation_id,
    FALSE as receive_updates_by_notification,
    FALSE as receive_updates_by_email, 
    FALSE as receive_similar_conversation_updates_by_email,
    FALSE as receive_similar_conversation_updates_by_notification
FROM user_participation up
JOIN workflow w ON up.workflow_id = w.id
WHERE w.conversation_id IS NOT NULL
AND NOT EXISTS (
    SELECT 1 
    FROM user_conversation_preferences ucp 
    WHERE ucp.user_id = up.user_id 
    AND ucp.conversation_id = w.conversation_id
);