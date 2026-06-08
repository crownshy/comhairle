-- Add location column to event table

ALTER TABLE event ADD COLUMN location JSONB;
