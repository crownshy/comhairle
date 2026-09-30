-- Create scheduled_email table

CREATE TABLE scheduled_email (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    user_email TEXT NOT NULL,
    email_config JSONB NOT NULL,
    status TEXT NOT NULL DEFAULT 'pending',
    send_at TIMESTAMPTZ NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX scheduled_email_status_send_at_index ON scheduled_email(status, send_at)
WHERE status = 'pending';
