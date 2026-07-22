CREATE TABLE IF NOT EXISTS live_audio_recording_multipart_upload (
    live_audio_recording_id uuid PRIMARY KEY REFERENCES live_audio_recording(id) ON DELETE CASCADE,
    multipart_upload_id TEXT NOT NULL,
    next_part_number INTEGER NOT NULL DEFAULT 1,
    uploaded_parts JSONB NOT NULL DEFAULT '[]'::jsonb,

    CONSTRAINT live_audio_recording_multipart_next_part_positive CHECK (next_part_number > 0)
);

INSERT INTO live_audio_recording_multipart_upload (
    live_audio_recording_id,
    multipart_upload_id,
    next_part_number,
    uploaded_parts
)
SELECT
    id,
    multipart_upload_id,
    next_part_number,
    uploaded_parts
FROM live_audio_recording
ON CONFLICT (live_audio_recording_id) DO NOTHING;

ALTER TABLE live_audio_recording
    DROP CONSTRAINT IF EXISTS live_audio_recording_next_part_positive;

ALTER TABLE live_audio_recording
    DROP COLUMN IF EXISTS multipart_upload_id,
    DROP COLUMN IF EXISTS next_part_number,
    DROP COLUMN IF EXISTS uploaded_parts;
