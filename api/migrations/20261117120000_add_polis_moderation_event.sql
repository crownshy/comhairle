-- An append-only record of every moderation decision on a Polis statement, so decisions
-- can be checked later. A statement row only keeps its current status; this keeps how
-- it got there. An undo is a new row, never an edit.

CREATE TABLE polis_moderation_event (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    polis_statement_aux_id UUID NOT NULL REFERENCES polis_statement_aux(id) ON DELETE CASCADE,
    workflow_step_id UUID NOT NULL REFERENCES workflow_step(id) ON DELETE CASCADE,
    actor_id UUID REFERENCES comhairle_user(id) ON DELETE SET NULL,
    action TEXT NOT NULL CHECK (action IN ('decision', 'split', 'undo_split')),
    from_status TEXT NOT NULL,
    to_status TEXT NOT NULL,
    reason TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_polis_moderation_event_workflow_step_id
    ON polis_moderation_event(workflow_step_id, created_at DESC);

CREATE FUNCTION polis_moderation_event_is_append_only() RETURNS trigger AS $$
BEGIN
    RAISE EXCEPTION 'polis_moderation_event rows are never edited';
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER polis_moderation_event_no_update
    BEFORE UPDATE ON polis_moderation_event
    FOR EACH ROW EXECUTE FUNCTION polis_moderation_event_is_append_only();
