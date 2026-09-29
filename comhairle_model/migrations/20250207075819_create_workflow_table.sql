-- Add migration script here
CREATE TABLE workflow(
    id uuid PRIMARY KEY DEFAULT uuid_generate_v4(),
    conversation_id uuid REFERENCES conversation(id),
    name Text,
    description Text,
    is_active bool DEFAULT FALSE,
    is_public bool DEFAULT FALSE,
    owner_id uuid REFERENCES comhairle_user,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
)
