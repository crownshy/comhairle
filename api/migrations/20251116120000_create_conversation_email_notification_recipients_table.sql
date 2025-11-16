-- Add migration script here
-- Create conversation_email_notification_recipients table to store email addresses for conversation updates for non-logged-in users

CREATE TABLE conversation_email_notification_recipients (
    id uuid PRIMARY KEY DEFAULT uuid_generate_v4(),
    conversation_id uuid NOT NULL REFERENCES conversation(id) ON DELETE CASCADE,
    email VARCHAR(255) NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Unique constraint to ensure one email can only be registered once per conversation
CREATE UNIQUE INDEX conversation_email_notification_recipients_unique_index ON conversation_email_notification_recipients(conversation_id, email);

-- Index on conversation_id for querying all email recipients for a conversation
CREATE INDEX conversation_email_notification_recipients_conversation_id_index ON conversation_email_notification_recipients(conversation_id);

-- Index on email for looking up subscriptions by email address
CREATE INDEX conversation_email_notification_recipients_email_index ON conversation_email_notification_recipients(email);