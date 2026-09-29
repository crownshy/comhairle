-- Demographics questions
CREATE TYPE demographics_response_type AS ENUM ('string', 'number');

CREATE TABLE demographics_question (
    slug TEXT NOT NULL PRIMARY KEY,
    response_type demographics_response_type NOT NULL
);

CREATE INDEX idx_demographics_question_slug ON demographics_question(slug);

-- Demographics responses
CREATE TABLE demographics_response (
    id UUID NOT NULL PRIMARY KEY DEFAULT uuid_generate_v4(),
    question_slug TEXT NOT NULL REFERENCES demographics_question(slug) ON DELETE CASCADE,
    user_id UUID REFERENCES comhairle_user(id) ON DELETE SET NULL,
    value TEXT NOT NULL,
    CONSTRAINT chk_value_not_empty CHECK (value <> '')
);

CREATE INDEX idx_demographics_response_demographics_question ON demographics_response(question_slug);
CREATE INDEX idx_demographics_response_user_id ON demographics_response(user_id);

-- {question_slug, user_id} should be unique if user_id is not null (user account is not yet deleted)
CREATE UNIQUE INDEX IF NOT EXISTS uq_demographics_response_question_slug_user_id ON demographics_response (question_slug, user_id) 
WHERE user_id IS NOT NULL;

-- Let's add the age, ethnicity, gender, location, and political party demographics questions.
INSERT INTO demographics_question (slug, response_type) VALUES
    ('age', 'number'),
    ('ethnicity', 'string'),
    ('gender', 'string'),
    ('zipcode', 'string'),
    ('political_party', 'string');

-- Let's migrate the existing age, ethnicity, gender, and political party data from the user_profile table to the new demographics_response table.
-- Migrate one field at a time
-- Migrate age
INSERT INTO demographics_response (question_slug, user_id, value)
SELECT 'age', user_id, age::TEXT
FROM user_profile
WHERE age IS NOT NULL;

-- Migrate ethnicity
INSERT INTO demographics_response (question_slug, user_id, value)
SELECT 'ethnicity', user_id, ethnicity
FROM user_profile
WHERE ethnicity IS NOT NULL;

-- Migrate gender
INSERT INTO demographics_response (question_slug, user_id, value)
SELECT 'gender', user_id, gender
FROM user_profile
WHERE gender IS NOT NULL;

-- Migrate zipcode
INSERT INTO demographics_response (question_slug, user_id, value)
SELECT 'zipcode', user_id, zipcode
FROM user_profile
WHERE zipcode IS NOT NULL;

-- Migrate political_party
INSERT INTO demographics_response (question_slug, user_id, value)
SELECT 'political_party', user_id, political_party
FROM user_profile
WHERE political_party IS NOT NULL;

-- Delete the migrated columns from the user_profile table.
ALTER TABLE user_profile
DROP COLUMN IF EXISTS age,
DROP COLUMN IF EXISTS ethnicity,
DROP COLUMN IF EXISTS gender,
DROP COLUMN IF EXISTS zipcode,
DROP COLUMN IF EXISTS political_party;

-- Many-many relationship between conversations and demographics questions.
CREATE TABLE conversation_demographics (
    conversation_id UUID NOT NULL REFERENCES conversation(id) ON DELETE CASCADE,
    question_slug TEXT NOT NULL REFERENCES demographics_question(slug) ON DELETE CASCADE,
    CONSTRAINT pk_conversation_demographics PRIMARY KEY (conversation_id, question_slug)
);

CREATE INDEX idx_conversation_demographics_conversation_id ON conversation_demographics(conversation_id);
CREATE INDEX idx_conversation_demographics_question_slug ON conversation_demographics(question_slug);

-- Create demographics relationships from existing metadata
INSERT INTO conversation_demographics (conversation_id, question_slug)
SELECT c.id, q.slug
FROM conversation c
JOIN demographics_question q ON q.slug IN ('age', 'ethnicity', 'gender', 'zipcode', 'political_party')
WHERE c.metadata->'demographics'->>q.slug = 'true';

UPDATE conversation
SET metadata = metadata - 'demographics'
WHERE metadata ? 'demographics';
