-- Add region areas.
CREATE TABLE region_area (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    zip_prefix TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE UNIQUE INDEX IF NOT EXISTS region_area_zip_prefix_unique_index
ON region_area(zip_prefix);

-- Add region area and metadata to regions.
ALTER TABLE region
ADD COLUMN region_area_id UUID REFERENCES region_area(id),
ADD COLUMN metadata JSONB;

CREATE INDEX IF NOT EXISTS region_region_area_id_index
ON region(region_area_id);

-- Add metadata to organizations.
ALTER TABLE organization
ADD COLUMN metadata JSONB;

-- Add metadata to events.
ALTER TABLE event
ADD COLUMN metadata JSONB;
