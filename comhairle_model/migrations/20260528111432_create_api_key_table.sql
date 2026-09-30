-- Create api_key table

CREATE TABLE api_key (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    hash TEXT NOT NULL,
    user_id UUID NOT NULL REFERENCES comhairle_user(id) ON DELETE CASCADE,
    name TEXT NOT NULL,
    prefix TEXT NOT NULL,
    revoked_at TIMESTAMPTZ,
    expired_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE UNIQUE INDEX api_key_hash ON api_key(hash);
