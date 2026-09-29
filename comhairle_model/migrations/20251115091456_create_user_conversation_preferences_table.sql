-- Add migration script here
-- Create user_conversation_preferences table to store user notification preferences per conversation

CREATE TABLE user_conversation_preferences (
    id uuid PRIMARY KEY DEFAULT uuid_generate_v4(),
    user_id uuid NOT NULL REFERENCES comhairle_user(id) ON DELETE CASCADE,
    conversation_id uuid NOT NULL REFERENCES conversation(id) ON DELETE CASCADE,
    receive_updates_by_notification BOOLEAN NOT NULL DEFAULT FALSE,
    receive_updates_by_email BOOLEAN NOT NULL DEFAULT FALSE,
    receive_similar_conversation_updates_by_email BOOLEAN NOT NULL DEFAULT FALSE,
    receive_similar_conversation_updates_by_notification BOOLEAN NOT NULL DEFAULT FALSE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Unique constraint to ensure one preference record per user per conversation
CREATE UNIQUE INDEX user_conversation_preferences_unique_index ON user_conversation_preferences(user_id, conversation_id);

-- Index on user_id for querying all preferences for a user
CREATE INDEX user_conversation_preferences_user_id_index ON user_conversation_preferences(user_id);

-- Index on conversation_id for querying all user preferences for a conversation
CREATE INDEX user_conversation_preferences_conversation_id_index ON user_conversation_preferences(conversation_id);
