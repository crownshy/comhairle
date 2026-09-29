-- Create organization table
CREATE TABLE organization (
    id uuid PRIMARY KEY DEFAULT uuid_generate_v4(),
    name TEXT NOT NULL,
    description UUID NOT NULL,
    mission UUID NOT NULL,
    org_type TEXT NOT NULL,
    external_url TEXT,
    regions UUID[] NOT NULL DEFAULT '{}',
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
