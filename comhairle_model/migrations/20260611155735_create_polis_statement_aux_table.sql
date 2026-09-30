-- Create polis_statement_aux table

CREATE TABLE polis_statement_aux (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    workflow_step_id UUID NOT NULL REFERENCES workflow_step(id) ON DELETE CASCADE,
    user_id UUID NOT NULL REFERENCES comhairle_user(id) ON DELETE CASCADE,
    zid INTEGER NOT NULL,
    polis_conversation_id TEXT NOT NULL,
    polis_statement_id INTEGER NOT NULL,
    themes TEXT[] NOT NULL DEFAULT '{}',
    visible_statement_when_submitted TEXT,
    moderation_reason TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
