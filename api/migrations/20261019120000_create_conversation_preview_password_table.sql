-- One shared password per conversation that lets people without an account open its preview.
-- Kept out of the conversation table so the hash can never ride along in a conversation DTO.
CREATE TABLE conversation_preview_password (
    conversation_id UUID PRIMARY KEY REFERENCES conversation(id) ON DELETE CASCADE,
    password_hash TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
