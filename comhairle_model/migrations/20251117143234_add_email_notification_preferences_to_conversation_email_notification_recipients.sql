-- Add migration script here
-- Add email notification preference fields to conversation_email_notification_recipients table

ALTER TABLE conversation_email_notification_recipients 
ADD COLUMN receive_updates_by_email BOOLEAN NOT NULL DEFAULT FALSE,
ADD COLUMN receive_similar_conversation_updates_by_email BOOLEAN NOT NULL DEFAULT FALSE;