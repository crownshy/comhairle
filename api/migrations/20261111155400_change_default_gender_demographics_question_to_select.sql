-- Change the default gender demographics question to be a select

UPDATE demographics_question_version
SET question_type = 'select', multi = false
WHERE question_id = 'de000001-0000-4000-8000-000000000003';

INSERT INTO text_content (id, primary_locale, format) VALUES
    ('de000005-0000-4000-8000-000000000007', 'en', 'plain'),
    ('de000005-0000-4000-8000-000000000008', 'en', 'plain'),
    ('de000005-0000-4000-8000-000000000009', 'en', 'plain');

INSERT INTO text_translation (content_id, locale, content, ai_generated, requires_validation) VALUES
    ('de000005-0000-4000-8000-000000000007', 'en', 'Male', false, false),
    ('de000005-0000-4000-8000-000000000008', 'en', 'Female', false, false),
    ('de000005-0000-4000-8000-000000000009', 'en', 'Non-binary', false, false);

INSERT INTO demographics_question_option (id, question_version_id, position, label_text_content_id, created_at, updated_at) VALUES
    ('de000001-0000-4000-8000-000000000001',  'de000002-0000-4000-8000-000000000003', 1, 'de000005-0000-4000-8000-000000000007', NOW(), NOW()),
    ('de000001-0000-4000-8000-000000000002',  'de000002-0000-4000-8000-000000000003', 2, 'de000005-0000-4000-8000-000000000008', NOW(), NOW()),
    ('de000001-0000-4000-8000-000000000003',  'de000002-0000-4000-8000-000000000003', 3, 'de000005-0000-4000-8000-000000000009', NOW(), NOW());
