-- Add format column to event table

ALTER TABLE event ADD COLUMN format TEXT NOT NULL DEFAULT 'online';
