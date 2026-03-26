-- Add thank_you_message column to conversation table
ALTER TABLE conversation
ADD COLUMN thank_you_message UUID;
