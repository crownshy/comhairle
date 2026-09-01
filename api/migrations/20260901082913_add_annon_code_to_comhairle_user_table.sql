-- Add annon_code column to comhairle_user table
ALTER TABLE comhairle_user
ADD COLUMN annon_code TEXT;

-- Copy username to annon_code for annon auth_type users
UPDATE comhairle_user
SET annon_code = username
WHERE auth_type = 'annon';

-- Remove unique constraint on username column
DROP INDEX IF EXISTS username_index;

-- Add unique constraint to annon_code
CREATE UNIQUE INDEX IF NOT EXISTS comhairle_user_annon_code_unique_index
ON comhairle_user(annon_code) WHERE auth_type = 'annon';

-- Add non-unique search index on username
CREATE INDEX IF NOT EXISTS comhairle_user_username_index
ON comhairle_user(username);
