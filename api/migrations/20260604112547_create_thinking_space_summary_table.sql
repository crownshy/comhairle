-- Create thinking_space_summary table

CREATE TABLE thinking_space_summary (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    workflow_step_id UUID REFERENCES workflow_step(id) ON DELETE CASCADE,
    user_id UUID REFERENCES comhairle_user(id) ON DELETE CASCADE,
    summary TEXT NOT NULL,
    is_ai_generated BOOLEAN DEFAULT FALSE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
