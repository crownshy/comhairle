-- Add GIN index to organization regions
CREATE INDEX IF NOT EXISTS organization_regions_index
ON organization
USING gin(regions);
