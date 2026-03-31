-- Add short_privacy_policy column to conversation table
ALTER TABLE conversation
ADD COLUMN short_privacy_policy UUID;

