-- Add migration script here

CREATE TABLE user_progress(
    id uuid PRIMARY KEY DEFAULT uuid_generate_v4(),
    user_id uuid References comhairle_user NOT NULL,
    workflow_step_id uuid References workflow_step NOT NULL,
    status TEXT NOT NULL DEFAULT 'not_started',

    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    
    UNIQUE (user_id, workflow_step_id) DEFERRABLE INITIALLY DEFERRED
)
