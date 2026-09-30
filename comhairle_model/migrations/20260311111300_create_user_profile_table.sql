-- Add migration script here
-- Create user_profile table to store user demographic information with consent

CREATE TABLE user_profile (
    id uuid PRIMARY KEY DEFAULT uuid_generate_v4(),
    user_id uuid NOT NULL REFERENCES comhairle_user(id) ON DELETE CASCADE,
    consented BOOLEAN NOT NULL,
    ethnicity TEXT DEFAULT NULL,
    age INTEGER DEFAULT NULL,
    gender TEXT DEFAULT NULL,
    zipcode TEXT DEFAULT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Unique constraint to ensure one profile per user
CREATE UNIQUE INDEX user_profile_user_id_unique ON user_profile(user_id);

-- Index for querying profiles by user
CREATE INDEX user_profile_user_index ON user_profile(user_id);

-- Index for querying profiles where users have consented
CREATE INDEX user_profile_consented_index ON user_profile(consented) WHERE consented = true;
