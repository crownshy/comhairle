-- Create audio recording table to track audio uploads for events
-- One record per event with all breakout room IDs and overall status
CREATE TABLE audio_recording (
    id uuid PRIMARY KEY DEFAULT uuid_generate_v4(),
    event_id uuid NOT NULL UNIQUE REFERENCES event(id) ON DELETE CASCADE,
    breakout_room_ids TEXT[] NOT NULL DEFAULT '{}',
    s3_key_prefix TEXT NOT NULL,
    status TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Add index for querying recordings by event
CREATE INDEX idx_audio_recording_event_id ON audio_recording(event_id);
