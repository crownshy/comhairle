-- Add verified column to comhairle_user table

ALTER TABLE comhairle_user
ADD COLUMN email_verified BOOLEAN NOT NULL DEFAULT FALSE;
