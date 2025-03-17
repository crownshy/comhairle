-- Add migration script here
CREATE TABLE workflow_step(
    id uuid PRIMARY KEY DEFAULT uuid_generate_v4(),
    workflow_id uuid REFERENCES workflow,
    name  Text,
    step_order INT,
    activation_rule JSONB,
    description  Text,
    is_offline boolean DEFAULT false,
    tool_config  JSONB,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE (workflow_id, step_order) DEFERRABLE INITIALLY DEFERRED
)

