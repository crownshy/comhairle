-- Remove unique index on username column
DROP INDEX IF EXISTS username_index;

-- Re-add unique index on username column for non-guest and non-one-time-password users
CREATE UNIQUE INDEX username_index ON comhairle_user (username)
WHERE
    auth_type != 'guest'
    AND auth_type != 'one_time_passcode';