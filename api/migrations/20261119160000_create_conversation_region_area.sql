CREATE TABLE conversation_region_area (
    conversation_id UUID NOT NULL REFERENCES conversation(id) ON DELETE CASCADE,
    region_area_id UUID NOT NULL REFERENCES region_area(id) ON DELETE CASCADE,
    PRIMARY KEY (conversation_id, region_area_id)
);

CREATE INDEX conversation_region_area_region_area_id_index
ON conversation_region_area(region_area_id);
