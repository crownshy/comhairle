-- Add display_name and bucket_config columns to demographics_question table
ALTER TABLE demographics_question
    ADD COLUMN display_name TEXT,
    ADD COLUMN bucket_config JSONB;

-- Set display_name for existing demographics questions
UPDATE demographics_question SET display_name = 'Age' WHERE slug = 'age';
UPDATE demographics_question SET display_name = 'Ethnicity' WHERE slug = 'ethnicity';
UPDATE demographics_question SET display_name = 'Gender' WHERE slug = 'gender';
UPDATE demographics_question SET display_name = 'Zipcode' WHERE slug = 'zipcode';
UPDATE demographics_question SET display_name = 'Political Party' WHERE slug = 'political_party';

-- Set display name as not null
ALTER TABLE demographics_question
    ALTER COLUMN display_name SET NOT NULL;

-- Set bucket_config for existing age question
UPDATE demographics_question
SET bucket_config = '{
    "type": "numeric",
    "buckets": [
        {"label": "Under 18", "min": null, "max": 17},
        {"label": "18-24", "min": 18, "max": 24},
        {"label": "25-34", "min": 25, "max": 34},
        {"label": "35-44", "min": 35, "max": 44},
        {"label": "45-54", "min": 45, "max": 54},
        {"label": "55-64", "min": 55, "max": 64},
        {"label": "65+", "min": 65, "max": null}
    ]
}'::jsonb
WHERE slug = 'age';
