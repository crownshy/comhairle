-- Add privacy_policy column to conversation table
ALTER TABLE conversation
ADD COLUMN privacy_policy UUID;
