-- Make audio_recording per-room instead of per-event.
--
-- Each row now represents a single named room within an event, and an event may
-- have many rooms. The previous model kept one row per event with the breakout
-- rooms stored in a TEXT[] array; that distinction (main vs. breakout) is gone.
--
-- The existing data is from a throwaway prototype with no users, so we discard it
-- up front. That lets us add a NOT NULL `name` column without a backfill.
DELETE FROM audio_recording;

-- Many rooms per event now, so the event_id can no longer be unique on its own.
ALTER TABLE audio_recording DROP CONSTRAINT audio_recording_event_id_key;

-- The main-vs-breakout concept is gone; rooms are tracked as their own rows.
ALTER TABLE audio_recording DROP COLUMN breakout_room_ids;

-- User-supplied room name, unique within an event.
ALTER TABLE audio_recording ADD COLUMN name TEXT NOT NULL;
ALTER TABLE audio_recording
    ADD CONSTRAINT audio_recording_event_id_name_key UNIQUE (event_id, name);
