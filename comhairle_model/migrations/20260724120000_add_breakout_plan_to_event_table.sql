-- Add breakout_plan column to event table to store pre-assigned breakout rooms.
-- Shape: JSON array of rooms, each room an array of seats. A seat references
-- either a known attendee (user_id) or a reserved placeholder for an invite
-- that has not signed up yet (invite_id).

ALTER TABLE event
ADD COLUMN breakout_plan JSONB NOT NULL DEFAULT '[]'::jsonb;
