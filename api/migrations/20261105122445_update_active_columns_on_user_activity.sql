-- Add is_active column to user_activity table
-- Replace last_active_at with activity_changed_at timestamp

ALTER TABLE user_activity
DROP COLUMN IF EXISTS last_active_at,
ADD COLUMN activity_changed_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
ADD COLUMN is_active boolean NOT NULL DEFAULT true;

DROP INDEX IF EXISTS user_activity_active_now;

CREATE INDEX user_activity_active_now ON user_activity(activity_changed_at) WHERE ended_at IS NULL;

