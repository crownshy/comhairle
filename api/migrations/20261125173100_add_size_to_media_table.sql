-- Add size column to media to be able to show the file size to the user in the media library

ALTER TABLE media
ADD COLUMN size INTEGER NOT NULL DEFAULT 0;
