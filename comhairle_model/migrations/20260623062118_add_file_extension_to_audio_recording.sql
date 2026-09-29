-- Track the file extension (and therefore the audio format) of the uploaded
-- recording so the transcription pipeline can support formats other than WAV.
ALTER TABLE audio_recording
    ADD COLUMN file_extension TEXT NOT NULL DEFAULT 'wav';
