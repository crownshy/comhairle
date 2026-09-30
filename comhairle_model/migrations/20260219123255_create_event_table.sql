-- Create event table to store video conference events

CREATE TABLE event (
    id uuid PRIMARY KEY DEFAULT uuid_generate_v4(),
    name uuid NOT NULL,
    description uuid NOT NULL,
    capacity INT,
    conversation_id uuid NOT NULL REFERENCES conversation(id),
    start_time TIMESTAMPTZ NOT NULL,
    end_time TIMESTAMPTZ NOT NULL,
    signup_mode TEXT NOT NULL DEFAULT 'invite',
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

ALTER TABLE event
ADD CONSTRAINT signup_mode_check
CHECK (signup_mode in ('invite', 'open'));
