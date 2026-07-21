-- Proposals move from a single `body` to an ordered list of translatable sections.
-- Each section lives in its own row with a stable id and its own translatable
-- text content, so it can be reordered and translated independently.

CREATE TABLE proposal_evaluation_proposal_section (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    proposal_id UUID NOT NULL REFERENCES proposal_evaluation_proposal(id) ON DELETE CASCADE,
    position INTEGER NOT NULL DEFAULT 0,
    body UUID,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_proposal_section_proposal_id
    ON proposal_evaluation_proposal_section (proposal_id);

-- Migrate each existing proposal's single body into a first section (position 0),
-- reusing the existing text_content reference so no translations are lost.
INSERT INTO proposal_evaluation_proposal_section (proposal_id, position, body)
SELECT id, 0, body
FROM proposal_evaluation_proposal
WHERE body IS NOT NULL;

-- Drop the now-superseded single body column.
ALTER TABLE proposal_evaluation_proposal DROP COLUMN body;
