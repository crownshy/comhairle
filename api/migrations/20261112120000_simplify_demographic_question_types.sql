ALTER TABLE demographics_question_version
    DROP CONSTRAINT demographics_question_version_question_type_check,
    DROP CONSTRAINT chk_question_version_location_format,
    DROP CONSTRAINT chk_question_version_json_schema,
    DROP COLUMN location_format,
    DROP COLUMN location_country,
    DROP COLUMN json_schema;

-- Keep version identities and submitted answers unchanged, including published versions.
UPDATE demographics_question_version
SET question_type = 'text'
WHERE question_type IN ('any_string', 'location', 'custom');

ALTER TABLE demographics_question_version
    ADD CONSTRAINT demographics_question_version_question_type_check CHECK (
        question_type IN ('number', 'single_select', 'multi_select', 'date', 'text')
    );
