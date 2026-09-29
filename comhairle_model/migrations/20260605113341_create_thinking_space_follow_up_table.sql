-- Create thinking_space_follow_up_question table

CREATE TABLE thinking_space_follow_up_question (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    user_id UUID NOT NULL REFERENCES comhairle_user(id) ON DELETE CASCADE,
    workflow_step_id UUID NOT NULL REFERENCES workflow_step(id) ON DELETE CASCADE,
    root_question_id UUID NOT NULL,
    follow_up_questions TEXT[] NOT NULL DEFAULT '{}',
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
