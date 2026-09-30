-- Rename audio_recording.status values so each name describes the action
-- happening at that state. Also makes room for an `awaiting_upload` initial
-- state (set in code) that distinguishes "row created, file not yet received"
-- from "we have the file and are transcribing".
UPDATE audio_recording SET status = 'transcribing'         WHERE status = 'pending';
UPDATE audio_recording SET status = 'categorizing'         WHERE status = 'transcript_available';
UPDATE audio_recording SET status = 'complete'             WHERE status = 'both_available';
UPDATE audio_recording SET status = 'transcription_failed' WHERE status = 'transcript_failure';
UPDATE audio_recording SET status = 'categorization_failed' WHERE status = 'categorization_failure';
