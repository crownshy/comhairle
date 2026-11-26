-- Fix typo in launch_date column (unused up until this point)

ALTER TABLE conversation
DROP COLUMN lauch_date,
ADD COLUMN launch_date TIMESTAMPTZ;
