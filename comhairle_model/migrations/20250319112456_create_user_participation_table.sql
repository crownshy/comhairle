-- Add migration script here

CREATE TABLE user_participation(
    id uuid PRIMARY KEY DEFAULT uuid_generate_v4(),
    user_id uuid References comhairle_user NOT NULL,
    workflow_id uuid References workflow NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE (user_id, workflow_id) DEFERRABLE INITIALLY DEFERRED
)
