-- Add name column to the media table
ALTER TABLE media
ADD COLUMN name TEXT;

UPDATE media
SET name = filename;

ALTER TABLE media
ALTER COLUMN name SET NOT NULL;
