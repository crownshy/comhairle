CREATE TABLE live_audio_recording (
    id uuid PRIMARY KEY DEFAULT uuid_generate_v4(),
    audio_recording_id uuid NOT NULL UNIQUE REFERENCES audio_recording(id) ON DELETE CASCADE,
    multipart_upload_id TEXT NOT NULL,
    next_part_number INTEGER NOT NULL DEFAULT 1,
    uploaded_parts JSONB NOT NULL DEFAULT '[]'::jsonb,
    locked_by_user_id uuid REFERENCES comhairle_user(id) ON DELETE SET NULL,
    CONSTRAINT live_audio_recording_next_part_positive CHECK (next_part_number > 0)
);
