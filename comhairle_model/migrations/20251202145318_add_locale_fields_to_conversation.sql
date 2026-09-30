-- Add migration script here
-- Add primary_locale and supported_languages fields to conversation table

ALTER TABLE conversation 
ADD COLUMN primary_locale TEXT NOT NULL DEFAULT 'en',
ADD COLUMN supported_languages TEXT[] NOT NULL DEFAULT ARRAY['en'];

-- Create an index on primary_locale for better query performance
CREATE INDEX idx_conversation_primary_locale ON conversation(primary_locale);