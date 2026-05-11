-- Add event_id column to invite table
ALTER TABLE invite ADD COLUMN event_id UUID REFERENCES event(id);

