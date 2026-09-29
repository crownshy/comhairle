-- Add guest_code column to comhairle_user table
ALTER TABLE comhairle_user
ADD COLUMN guest_code TEXT;

-- Copy username to guest_code for annon auth_type users
UPDATE comhairle_user
SET guest_code = username
WHERE auth_type = 'annon';

-- Update auth_type = 'annon' to 'guest'
UPDATE comhairle_user
SET auth_type = 'guest'
WHERE auth_type = 'annon';

-- Remove unique index on username column
DROP INDEX IF EXISTS username_index;

-- Re-add unique index on username column for non-guest users
CREATE UNIQUE INDEX username_index
ON comhairle_user(username)
WHERE auth_type != 'guest';

-- Add unique index to guest_code
CREATE UNIQUE INDEX IF NOT EXISTS comhairle_user_guest_code_unique_index
ON comhairle_user(guest_code)
WHERE auth_type = 'guest';
