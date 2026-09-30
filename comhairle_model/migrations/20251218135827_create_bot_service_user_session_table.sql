-- Add table to store user sessions for bot service chat assistant sessions
-- Should be unique to conversation and user

CREATE TABLE bot_service_user_session (
    id uuid PRIMARY KEY DEFAULT uuid_generate_v4(),
    user_id UUID NOT NULL,
    conversation_id UUID NOT NULL,
    bot_service_session_id TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    CONSTRAINT bot_session_conversation_id_fk
    FOREIGN KEY (conversation_id)
    REFERENCES conversation (id)
    ON DELETE CASCADE,

    CONSTRAINT bot_session_user_id_fk
    FOREIGN KEY (user_id)
    REFERENCES comhairle_user (id)
    ON DELETE CASCADE
);

CREATE UNIQUE INDEX bot_session_user_conversation_index
ON bot_service_user_session (user_id, conversation_id);
