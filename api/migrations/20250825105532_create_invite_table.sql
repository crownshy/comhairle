-- Add migration script here
CREATE TABLE invite(
    id uuid PRIMARY KEY DEFAULT uuid_generate_v4(),
	invite_type JSONB,
	created_by uuid REFERENCES  comhairle_user,
	status TEXT,
	expires_at TIMESTAMPTZ DEFAULT NULL,

	conversation_id uuid REFERENCES conversation NOT NULL,
	workflow_id uuid REFERENCES workflow,
	workflow_step_id uuid REFERENCES workflow_step,
	login_behaviour TEXT NOT NULL,
	tags  TEXT[] DEFAULT array[]::TEXT[],

	accept_count INTEGER DEFAULT 0,

    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
)

