-- Create jobs table for background workers

CREATE TABLE IF NOT EXISTS jobs (
    id uuid PRIMARY KEY DEFAULT uuid_generate_v4(),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    finished_at TIMESTAMPTZ DEFAULT NULL,
    error TEXT DEFAULT NULL,
    completion_message TEXT DEFAULT NULL,
    status TEXT DEFAULT 'pending'
);

ALTER TABLE jobs
ADD CONSTRAINT jobs_status_check
CHECK (status in ('pending', 'running', 'error', 'completed'));
