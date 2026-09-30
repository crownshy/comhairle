-- Add name column to the media table
ALTER TABLE media
ADD COLUMN IF NOT EXISTS alt TEXT;

UPDATE media
SET alt = 'temp';

ALTER TABLE media
ALTER COLUMN alt SET NOT NULL;
