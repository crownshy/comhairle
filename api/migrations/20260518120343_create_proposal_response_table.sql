-- Create table for proposal_evaluation_proposal_response

CREATE TABLE proposal_evalution_proposal_response (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    proposal_id UUID REFERENCES proposal_evaluation_proposal(id) ON DELETE CASCADE,
    user_id UUID REFERENCES comhairle_user(id),
    response JSONB NOT NULL
);
