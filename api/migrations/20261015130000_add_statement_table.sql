-- A statement is a piece of content attached to a workflow step, either written by a
-- participant (user_id set) or generated (ai_generated), with context explaining where
-- it came from.

CREATE TABLE statement (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    workflow_step_id UUID NOT NULL REFERENCES workflow_step(id) ON DELETE CASCADE,
    user_id UUID REFERENCES comhairle_user(id) ON DELETE SET NULL,
    content TEXT NOT NULL CHECK (btrim(content) <> ''),
    ai_generated BOOLEAN NOT NULL DEFAULT FALSE,
    context TEXT NOT NULL CHECK (btrim(context) <> ''),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_statement_workflow_step_id ON statement(workflow_step_id);
CREATE INDEX idx_statement_user_id ON statement(user_id);
