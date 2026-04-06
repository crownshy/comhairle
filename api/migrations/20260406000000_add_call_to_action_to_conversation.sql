-- Add call_to_action column to conversation table
ALTER TABLE conversation
ADD COLUMN call_to_action UUID;
