-- Add verified column to comhairle_user table

ALTER TABLE comhairle_user
ADD COLUMN verified BOOLEAN NOT NULL DEFAULT FALSE;
