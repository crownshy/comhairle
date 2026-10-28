-- Add table to track user activity analytics for workflow_steps and other future resources

CREATE TABLE user_activity (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    user_id UUID NOT NULL REFERENCES comhairle_user(id) ON DELETE CASCADE,
    workflow_step_id UUID NOT NULL REFERENCES workflow_step(id) ON DELETE CASCADE,
    started_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    last_active_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    ended_at TIMESTAMPTZ,
    active_seconds INT NOT NULL DEFAULT 0,
    sessions INT NOT NULL DEFAULT 1,
    end_reason TEXT
);

CREATE UNIQUE INDEX user_activity_user_step ON user_activity(user_id, workflow_step_id);
CREATE INDEX user_activity_step_started_at ON user_activity(workflow_step_id, started_at);
CREATE INDEX user_activity_active_now ON user_activity(last_active_at) WHERE ended_at IS NULL;
