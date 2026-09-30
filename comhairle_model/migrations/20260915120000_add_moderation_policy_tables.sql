-- A moderation policy groups the reasons a moderator can pick when rejecting a
-- statement. It belongs to a conversation, and a Polis step points at one through
-- `moderation_policy_id` in its tool config, so steps can share a policy.

CREATE TABLE moderation_policy (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    conversation_id UUID NOT NULL REFERENCES conversation(id) ON DELETE CASCADE,
    name TEXT NOT NULL CHECK (btrim(name) <> ''),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_moderation_policy_conversation_id ON moderation_policy(conversation_id);

-- Label uniqueness (case-insensitive) is checked in the API rather than with a unique
-- index, so a save that swaps two labels doesn't trip the index halfway through.
CREATE TABLE moderation_policy_reason (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    moderation_policy_id UUID NOT NULL REFERENCES moderation_policy(id) ON DELETE CASCADE,
    label TEXT NOT NULL CHECK (btrim(label) <> ''),
    description TEXT,
    position INTEGER NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_moderation_policy_reason_moderation_policy_id
    ON moderation_policy_reason(moderation_policy_id);
