-- A HeyForm survey attached to a conversation's thank-you page. Not a workflow step: it is
-- asked after the flow is finished, is optional, and stays out of "Step N of M".
CREATE TABLE conversation_feedback_survey (
    conversation_id UUID PRIMARY KEY REFERENCES conversation ON DELETE CASCADE,
    tool_config JSONB NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
