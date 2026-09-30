-- Create media table

CREATE TABLE media (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    store_name TEXT NOT NULL,
    storage_key TEXT NOT NULL,
    filename TEXT NOT NULL,
    content_type TEXT NOT NULL,
    owner_id UUID REFERENCES comhairle_user(id),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
