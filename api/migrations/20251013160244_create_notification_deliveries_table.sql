-- Add migration script here
-- Create notification_deliveries table to track delivery and read status per user

CREATE TABLE notification_deliveries (
    id uuid PRIMARY KEY DEFAULT uuid_generate_v4(),
    notification_id uuid NOT NULL REFERENCES notifications(id) ON DELETE CASCADE,
    user_id uuid NOT NULL REFERENCES comhairle_user(id) ON DELETE CASCADE,
    delivered_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    read_at TIMESTAMPTZ DEFAULT NULL,
    delivery_method TEXT NOT NULL DEFAULT 'in_app',
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Unique constraint to prevent duplicate deliveries to same user
CREATE UNIQUE INDEX notification_deliveries_unique_delivery 
ON notification_deliveries(notification_id, user_id);

-- Index for querying user's notifications
CREATE INDEX notification_deliveries_user_index ON notification_deliveries(user_id);

-- Index for querying notification deliveries
CREATE INDEX notification_deliveries_notification_index ON notification_deliveries(notification_id);

-- Index for querying unread notifications
CREATE INDEX notification_deliveries_unread_index ON notification_deliveries(user_id, read_at) 
WHERE read_at IS NULL;

-- Index on delivery method for filtering
CREATE INDEX notification_deliveries_method_index ON notification_deliveries(delivery_method);