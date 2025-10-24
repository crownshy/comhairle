-- Add migration script here
-- Create notifications table to store notification content

CREATE TABLE notifications (
    id uuid PRIMARY KEY DEFAULT uuid_generate_v4(),
    title TEXT NOT NULL,
    content TEXT NOT NULL,
    notification_type TEXT NOT NULL DEFAULT 'info',
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Index on notification_type for filtering
CREATE INDEX notifications_type_index ON notifications(notification_type);

-- Index on created_at for ordering
CREATE INDEX notifications_created_at_index ON notifications(created_at);