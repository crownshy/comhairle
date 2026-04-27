-- Add agenda column to event table to store structured agenda items

ALTER TABLE event
ADD COLUMN agenda JSONB NOT NULL DEFAULT '[]'::jsonb;
