-- Add chat_instructions table with additional prompt data for conversation
-- bot_service chat bots

CREATE TABLE IF NOT EXISTS chat_instructions (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    conversation_id UUID NOT NULL UNIQUE REFERENCES conversation(id) ON DELETE CASCADE,
    target_reading_age INTEGER CHECK (target_reading_age IS NULL OR target_reading_age BETWEEN 5 AND 18),
    max_length INTEGER CHECK (max_length IS NULL OR max_length > 0),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
