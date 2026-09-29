-- Add region_id to workflow table
ALTER TABLE workflow
ADD COLUMN region_id UUID REFERENCES region(id);
