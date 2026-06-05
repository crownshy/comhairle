-- Enforce one prioritization response per (proposal, user). Existing duplicates
-- are collapsed to the most recent row (latest created_at, with id as tiebreak)
-- so the unique index can be created safely.

DELETE FROM proposal_evalution_proposal_response a
USING proposal_evalution_proposal_response b
WHERE a.proposal_id = b.proposal_id
  AND a.user_id = b.user_id
  AND (
    a.created_at < b.created_at
    OR (a.created_at = b.created_at AND a.id < b.id)
  );

CREATE UNIQUE INDEX proposal_response_proposal_user_idx
    ON proposal_evalution_proposal_response (proposal_id, user_id);
