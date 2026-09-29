-- Create event attendance table to store user attendance for events

CREATE TABLE event_attendance (
    id uuid PRIMARY KEY DEFAULT uuid_generate_v4(),
    user_id uuid NOT NULL REFERENCES comhairle_user(id),
    event_id uuid NOT NULL REFERENCES event(id),
    role TEXT NOT NULL DEFAULT 'participant',
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE UNIQUE INDEX user_event_unique_index ON event_attendance(user_id, event_id);
