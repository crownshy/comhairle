-- Add multi column to the demographics_question_version table

ALTER TABLE demographics_question_version
ADD COLUMN "multi" BOOLEAN NOT NULL DEFAULT FALSE;

UPDATE demographics_question_version
SET multi = false;

ALTER TABLE demographics_question_version
DROP CONSTRAINT demographics_question_version_question_type_check;

UPDATE demographics_question_version
SET question_type = 'select', multi = true
WHERE question_type = 'multi_select';

UPDATE demographics_question_version
SET question_type = 'select', multi = false
WHERE question_type = 'single_select';

UPDATE demographics_question_version
SET question_type = 'text', multi = true
WHERE question_type = 'any_string';

ALTER TABLE demographics_question_version
ADD CONSTRAINT demographics_question_version_question_type_check
CHECK (question_type IN ('number', 'select', 'date', 'text', 'location', 'custom'));
