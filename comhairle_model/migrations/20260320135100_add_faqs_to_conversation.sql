-- Add faqs column to conversation table
ALTER TABLE conversation
ADD COLUMN faqs UUID;
