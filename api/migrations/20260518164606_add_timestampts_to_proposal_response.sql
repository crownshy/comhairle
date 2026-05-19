-- Add timestamp columns to proposal response

ALTER TABLE proposal_evalution_proposal_response
ADD COLUMN created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
ADD COLUMN updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW();
