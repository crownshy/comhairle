-- Add reminder_sent_at timestamp column to event table

ALTER TABLE event ADD COLUMN reminder_sent_at TIMESTAMPTZ;
