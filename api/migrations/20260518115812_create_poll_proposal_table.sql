-- Create table for proposal_evaluation_poll_proposal

CREATE TABLE proposal_evaluation_poll_proposal (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    workflow_step_id UUID REFERENCES workflow_step(id) ON DELETE CASCADE,
    title UUID,
    body UUID,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
