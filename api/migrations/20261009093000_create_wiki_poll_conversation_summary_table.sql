CREATE TABLE wiki_poll_conversation_summary (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    workflow_step_id UUID NOT NULL REFERENCES workflow_step(id) ON DELETE CASCADE,
    job_id UUID REFERENCES job(id) ON DELETE SET NULL,
    poll_data JSONB NOT NULL,
    result JSONB NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX wiki_poll_conversation_summary_workflow_step_id_idx
    ON wiki_poll_conversation_summary (workflow_step_id, created_at DESC);
