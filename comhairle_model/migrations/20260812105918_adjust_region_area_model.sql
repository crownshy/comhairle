--- Create a new table to represent the relationship between regions and region areas
CREATE TABLE region_region_area (
    region_id UUID NOT NULL REFERENCES region(id) ON DELETE CASCADE,
    region_area_id UUID NOT NULL REFERENCES region_area(id) ON DELETE CASCADE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    PRIMARY KEY (region_id, region_area_id)
);

CREATE INDEX IF NOT EXISTS region_region_area_region_id_index
ON region_region_area(region_id);

CREATE INDEX IF NOT EXISTS region_region_area_region_area_id_index
ON region_region_area(region_area_id);

-- Remove the region_area_id column from the regions table
ALTER TABLE region
DROP COLUMN IF EXISTS region_area_id;

-- Delete the region_region_area_id_index if it exists
DROP INDEX IF EXISTS region_region_area_id_index;
