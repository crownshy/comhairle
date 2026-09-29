-- Add migration script here
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE comhairle_user(
    id uuid PRIMARY KEY DEFAULT uuid_generate_v4(),
    auth_type TEXT NOT NULL,
    username TEXT DEFAULT NULL,
    email TEXT DEFAULT NULL,
    password TEXT,
    avatar_url TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);


CREATE UNIQUE INDEX username_index on comhairle_user(username);
CREATE UNIQUE INDEX email_index on comhairle_user(email);
