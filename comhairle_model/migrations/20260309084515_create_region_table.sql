-- Create region table
CREATE TABLE region (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    name UUID NOT NULL,
    description UUID NOT NULL,
    region_type TEXT NOT NULL,
    official_id TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

