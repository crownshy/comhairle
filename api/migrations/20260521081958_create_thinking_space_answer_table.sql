-- Create thinking_space_answer table

CREATE TABLE thinking_space_answer (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    workflow_step_id UUID REFERENCES workflow_step(id) ON DELETE CASCADE,
    user_id UUID REFERENCES comhairle_user(id) ON DELETE CASCADE,
    root_question_id UUID,
    is_follow_up boolean DEFAULT false,
    question TEXT NOT NULL,
    answer TEXT NOT NULL,
    other_questions TEXT[] NOT NULL DEFAULT '{}',
    status TEXT NOT NULL DEFAULT 'pending',
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
