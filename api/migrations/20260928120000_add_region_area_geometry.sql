CREATE EXTENSION IF NOT EXISTS postgis;

ALTER TABLE region_area
ALTER COLUMN zip_prefix DROP NOT NULL,
ADD COLUMN area_geometry geometry(MultiPolygon, 4326),
ADD COLUMN name TEXT,
ADD COLUMN tags TEXT[] NOT NULL DEFAULT '{}',
ADD COLUMN area_sqm DOUBLE PRECISION,
ADD COLUMN bbox_min_lng DOUBLE PRECISION,
ADD COLUMN bbox_min_lat DOUBLE PRECISION,
ADD COLUMN bbox_max_lng DOUBLE PRECISION,
ADD COLUMN bbox_max_lat DOUBLE PRECISION,
ADD CONSTRAINT region_area_geometry_valid CHECK (ST_IsValid(area_geometry));

CREATE INDEX region_area_geometry_index
ON region_area USING GIST (area_geometry);

CREATE INDEX region_area_tags_index
ON region_area USING GIN (tags);

-- Bounding box and area metrics for optimised viewport-based querying
CREATE FUNCTION region_area_set_metrics() RETURNS trigger AS $$
BEGIN
    IF NEW.area_geometry IS NULL THEN
        NEW.area_sqm := NULL;
        NEW.bbox_min_lng := NULL;
        NEW.bbox_min_lat := NULL;
        NEW.bbox_max_lng := NULL;
        NEW.bbox_max_lat := NULL;
    ELSE
        NEW.area_sqm := ST_Area(NEW.area_geometry::geography);
        NEW.bbox_min_lng := ST_XMin(NEW.area_geometry);
        NEW.bbox_min_lat := ST_YMin(NEW.area_geometry);
        NEW.bbox_max_lng := ST_XMax(NEW.area_geometry);
        NEW.bbox_max_lat := ST_YMax(NEW.area_geometry);
    END IF;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER region_area_metrics
BEFORE INSERT OR UPDATE ON region_area
FOR EACH ROW EXECUTE FUNCTION region_area_set_metrics();

UPDATE region_area
SET area_sqm = ST_Area(area_geometry::geography),
    bbox_min_lng = ST_XMin(area_geometry),
    bbox_min_lat = ST_YMin(area_geometry),
    bbox_max_lng = ST_XMax(area_geometry),
    bbox_max_lat = ST_YMax(area_geometry)
WHERE area_geometry IS NOT NULL;

CREATE INDEX region_area_bbox_index
ON region_area (bbox_min_lng, bbox_max_lng, bbox_min_lat, bbox_max_lat);

CREATE INDEX region_area_area_sqm_index
ON region_area (area_sqm);

CREATE UNIQUE INDEX region_area_unique_name_geom_idx 
ON region_area (
    name, 
    md5(ST_AsBinary(area_geometry))
)
WHERE area_geometry IS NOT NULL;