-- Who has finished the thank-you page feedback survey. Written when the embedded form reports
-- it is done, so the ask is not repeated, on any device. Goes with the survey when it is removed.
CREATE TABLE conversation_feedback_survey_completion (
    conversation_id UUID NOT NULL REFERENCES conversation_feedback_survey ON DELETE CASCADE,
    user_id UUID NOT NULL REFERENCES comhairle_user ON DELETE CASCADE,
    completed_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    PRIMARY KEY (conversation_id, user_id)
);
