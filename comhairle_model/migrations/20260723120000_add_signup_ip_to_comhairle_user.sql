-- Record the client IP address captured at account creation time.
-- Stored for internal/audit purposes only; never exposed via the API.

ALTER TABLE comhairle_user
ADD COLUMN signup_ip TEXT DEFAULT NULL;
