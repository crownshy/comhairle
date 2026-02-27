-- Add column to event table to hold ids for video meetings

ALTER TABLE event
ADD COLUMN video_meeting_id uuid;
