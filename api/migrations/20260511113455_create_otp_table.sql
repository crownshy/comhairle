-- Create one_time_passcode table to track and invalidate otps

CREATE TABLE one_time_passcode (
  id uuid PRIMARY KEY DEFAULT uuid_generate_v4(),
  user_id uuid REFERENCES comhairle_user(id),
  code TEXT NOT NULL,
  status TEXT DEFAULT 'pending',
  expires_at TIMESTAMPTZ NOT NULL,
  created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX otp_user_status_index ON one_time_passcode(user_id, status);
