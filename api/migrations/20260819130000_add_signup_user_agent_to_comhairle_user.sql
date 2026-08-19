-- Record the client browser signature (User-Agent) captured at account creation.
-- Stored for internal/audit purposes only; never exposed via the API.

ALTER TABLE comhairle_user
ADD COLUMN signup_user_agent TEXT DEFAULT NULL;
