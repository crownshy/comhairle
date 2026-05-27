-- Add default_time_zone column to event table

ALTER TABLE event
ADD COLUMN default_time_zone TEXT NOT NULL DEFAULT 'UTC';
