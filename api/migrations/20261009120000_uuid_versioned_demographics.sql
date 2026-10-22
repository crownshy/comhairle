-- Replaces the slug-keyed demographics model with UUID-identified, versioned
-- questions and forms. Question and form versions are immutable once published.

-- Move the slug-keyed tables aside so the new tables can reuse their names. Their
-- auto-named primary keys and indexes are renamed too, because those names are schema-wide.
-- They are read by the data migration below and dropped at the end of this file.
ALTER TABLE demographics_response RENAME TO legacy_demographics_response;
ALTER TABLE demographics_question RENAME TO legacy_demographics_question;
ALTER TABLE conversation_demographics RENAME TO legacy_conversation_demographics;
ALTER INDEX demographics_question_pkey RENAME TO legacy_demographics_question_pkey;
ALTER INDEX demographics_response_pkey RENAME TO legacy_demographics_response_pkey;
ALTER INDEX idx_demographics_question_slug RENAME TO legacy_idx_demographics_question_slug;
ALTER INDEX idx_demographics_response_demographics_question RENAME TO legacy_idx_demographics_response_question;
ALTER INDEX idx_demographics_response_user_id RENAME TO legacy_idx_demographics_response_user_id;
ALTER INDEX uq_demographics_response_question_slug_user_id RENAME TO legacy_uq_demographics_response_slug_user;
ALTER INDEX pk_conversation_demographics RENAME TO legacy_pk_conversation_demographics;
ALTER INDEX idx_conversation_demographics_conversation_id RENAME TO legacy_idx_conversation_demographics_conv;
ALTER INDEX idx_conversation_demographics_question_slug RENAME TO legacy_idx_conversation_demographics_slug;

-- Categories. A NULL organization_id marks a system tag shared by every organization.
CREATE TABLE demographics_tag (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    organization_id UUID REFERENCES organization(id) ON DELETE CASCADE,
    name TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    CONSTRAINT chk_demographics_tag_name_not_blank CHECK (btrim(name) <> '')
);

CREATE UNIQUE INDEX uq_demographics_tag_organization_name
    ON demographics_tag (organization_id, name) WHERE organization_id IS NOT NULL;
CREATE UNIQUE INDEX uq_demographics_tag_system_name
    ON demographics_tag (name) WHERE organization_id IS NULL;

-- A question's identity. Text, type and validation live on its versions.
-- A NULL organization_id marks a system default question.
CREATE TABLE demographics_question (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    organization_id UUID REFERENCES organization(id) ON DELETE CASCADE,
    forked_from_id UUID REFERENCES demographics_question(id) ON DELETE SET NULL,
    archived_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_demographics_question_organization_id ON demographics_question (organization_id);
CREATE INDEX idx_demographics_question_forked_from_id ON demographics_question (forked_from_id);

CREATE TABLE demographics_question_tag (
    question_id UUID NOT NULL REFERENCES demographics_question(id) ON DELETE CASCADE,
    tag_id UUID NOT NULL REFERENCES demographics_tag(id) ON DELETE CASCADE,
    PRIMARY KEY (question_id, tag_id)
);

CREATE INDEX idx_demographics_question_tag_tag_id ON demographics_question_tag (tag_id);

CREATE TABLE demographics_question_version (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    question_id UUID NOT NULL REFERENCES demographics_question(id) ON DELETE CASCADE,
    version_number INT NOT NULL CHECK (version_number >= 1),
    status TEXT NOT NULL CHECK (status IN ('draft', 'published')),
    text_content_id UUID NOT NULL REFERENCES text_content(id),
    question_type TEXT NOT NULL CHECK (
        question_type IN ('number', 'single_select', 'multi_select', 'date', 'any_string', 'location', 'custom')
    ),
    date_precision TEXT CHECK (date_precision IN ('day', 'month', 'year')),
    location_format TEXT CHECK (location_format IN ('postcode', 'lat_long', 'country')),
    location_country TEXT,
    max_selections INT CHECK (max_selections >= 1),
    allow_other BOOLEAN NOT NULL DEFAULT FALSE,
    prefer_not_to_say BOOLEAN NOT NULL DEFAULT FALSE,
    json_schema JSONB,
    special_category BOOLEAN NOT NULL DEFAULT FALSE,
    consent_text_content_id UUID REFERENCES text_content(id),
    published_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE (question_id, version_number),
    UNIQUE (id, question_id),
    CONSTRAINT chk_question_version_published_at CHECK ((status = 'published') = (published_at IS NOT NULL)),
    CONSTRAINT chk_question_version_date_precision CHECK ((question_type = 'date') = (date_precision IS NOT NULL)),
    CONSTRAINT chk_question_version_location_format CHECK ((question_type = 'location') = (location_format IS NOT NULL)),
    CONSTRAINT chk_question_version_json_schema CHECK ((question_type = 'custom') = (json_schema IS NOT NULL)),
    CONSTRAINT chk_question_version_max_selections CHECK (max_selections IS NULL OR question_type = 'multi_select'),
    CONSTRAINT chk_question_version_select_extras CHECK (
        question_type IN ('single_select', 'multi_select') OR (NOT allow_other AND NOT prefer_not_to_say)
    ),
    CONSTRAINT chk_question_version_consent_text CHECK (NOT special_category OR consent_text_content_id IS NOT NULL)
);

CREATE INDEX idx_demographics_question_version_question_id ON demographics_question_version (question_id);
CREATE UNIQUE INDEX uq_demographics_question_version_one_draft
    ON demographics_question_version (question_id) WHERE status = 'draft';

CREATE TABLE demographics_question_option (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    question_version_id UUID NOT NULL REFERENCES demographics_question_version(id) ON DELETE CASCADE,
    position INT NOT NULL CHECK (position >= 0),
    label_text_content_id UUID NOT NULL REFERENCES text_content(id),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE (question_version_id, position) DEFERRABLE INITIALLY DEFERRED
);

-- A form is an identity. Its name and question list live on its versions.
CREATE TABLE demographics_form (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    organization_id UUID NOT NULL REFERENCES organization(id) ON DELETE CASCADE,
    archived_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_demographics_form_organization_id ON demographics_form (organization_id);

CREATE TABLE demographics_form_version (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    form_id UUID NOT NULL REFERENCES demographics_form(id) ON DELETE CASCADE,
    version_number INT NOT NULL CHECK (version_number >= 1),
    status TEXT NOT NULL CHECK (status IN ('draft', 'published')),
    name TEXT NOT NULL CHECK (btrim(name) <> ''),
    published_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE (form_id, version_number),
    UNIQUE (id, form_id),
    CONSTRAINT chk_form_version_published_at CHECK ((status = 'published') = (published_at IS NOT NULL))
);

CREATE UNIQUE INDEX uq_demographics_form_version_one_draft
    ON demographics_form_version (form_id) WHERE status = 'draft';

-- question_version_id is pinned when the form version is published and stays NULL on drafts.
CREATE TABLE demographics_form_version_question (
    form_version_id UUID NOT NULL,
    form_id UUID NOT NULL,
    question_id UUID NOT NULL REFERENCES demographics_question(id),
    question_version_id UUID,
    position INT NOT NULL CHECK (position >= 0),
    required BOOLEAN NOT NULL DEFAULT TRUE,
    PRIMARY KEY (form_version_id, question_id),
    UNIQUE (form_version_id, position) DEFERRABLE INITIALLY DEFERRED,
    FOREIGN KEY (form_version_id, form_id) REFERENCES demographics_form_version(id, form_id) ON DELETE CASCADE,
    FOREIGN KEY (question_version_id, question_id)
        REFERENCES demographics_question_version(id, question_id)
);

-- One answer per participant per question version. Reused by every conversation
-- that asks that version. conversation_id records where it was last given.
CREATE TABLE demographics_response (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    question_version_id UUID NOT NULL REFERENCES demographics_question_version(id) ON DELETE CASCADE,
    user_id UUID REFERENCES comhairle_user(id) ON DELETE SET NULL,
    conversation_id UUID REFERENCES conversation(id) ON DELETE SET NULL,
    value JSONB NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE UNIQUE INDEX uq_demographics_response_version_user
    ON demographics_response (question_version_id, user_id) WHERE user_id IS NOT NULL;
CREATE INDEX idx_demographics_response_conversation_id ON demographics_response (conversation_id);

-- Consent to a special category question version. A declined consent (granted = false)
-- is kept so the question is not asked again.
CREATE TABLE demographics_consent (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    question_version_id UUID NOT NULL REFERENCES demographics_question_version(id) ON DELETE CASCADE,
    user_id UUID REFERENCES comhairle_user(id) ON DELETE SET NULL,
    granted BOOLEAN NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE UNIQUE INDEX uq_demographics_consent_version_user
    ON demographics_consent (question_version_id, user_id) WHERE user_id IS NOT NULL;

-- System default questions. Fixed UUIDs let code refer to them (see demographics::defaults).
INSERT INTO demographics_tag (id, organization_id, name) VALUES
    ('de000004-0000-4000-8000-000000000001', NULL, 'Age'),
    ('de000004-0000-4000-8000-000000000002', NULL, 'Ethnicity'),
    ('de000004-0000-4000-8000-000000000003', NULL, 'Gender'),
    ('de000004-0000-4000-8000-000000000004', NULL, 'Postcode'),
    ('de000004-0000-4000-8000-000000000005', NULL, 'Political affiliation');

INSERT INTO text_content (id, primary_locale, format) VALUES
    ('de000005-0000-4000-8000-000000000001', 'en', 'plain'),
    ('de000005-0000-4000-8000-000000000002', 'en', 'plain'),
    ('de000005-0000-4000-8000-000000000003', 'en', 'plain'),
    ('de000005-0000-4000-8000-000000000004', 'en', 'plain'),
    ('de000005-0000-4000-8000-000000000005', 'en', 'plain'),
    ('de000005-0000-4000-8000-000000000006', 'en', 'plain');

INSERT INTO text_translation (content_id, locale, content, ai_generated, requires_validation) VALUES
    ('de000005-0000-4000-8000-000000000001', 'en', 'What is your age?', false, false),
    ('de000005-0000-4000-8000-000000000002', 'en', 'What is your ethnicity?', false, false),
    ('de000005-0000-4000-8000-000000000003', 'en', 'What is your gender?', false, false),
    ('de000005-0000-4000-8000-000000000004', 'en', 'What is your postcode?', false, false),
    ('de000005-0000-4000-8000-000000000005', 'en', 'What is your political affiliation?', false, false),
    ('de000005-0000-4000-8000-000000000006', 'en',
        'Ethnicity is special category data. We only ask for it with your consent, and we use it to understand who takes part.',
        false, false);

INSERT INTO demographics_question (id, organization_id) VALUES
    ('de000001-0000-4000-8000-000000000001', NULL),
    ('de000001-0000-4000-8000-000000000002', NULL),
    ('de000001-0000-4000-8000-000000000003', NULL),
    ('de000001-0000-4000-8000-000000000004', NULL),
    ('de000001-0000-4000-8000-000000000005', NULL);

INSERT INTO demographics_question_tag (question_id, tag_id) VALUES
    ('de000001-0000-4000-8000-000000000001', 'de000004-0000-4000-8000-000000000001'),
    ('de000001-0000-4000-8000-000000000002', 'de000004-0000-4000-8000-000000000002'),
    ('de000001-0000-4000-8000-000000000003', 'de000004-0000-4000-8000-000000000003'),
    ('de000001-0000-4000-8000-000000000004', 'de000004-0000-4000-8000-000000000004'),
    ('de000001-0000-4000-8000-000000000005', 'de000004-0000-4000-8000-000000000005');

INSERT INTO demographics_question_version (
    id, question_id, version_number, status, text_content_id, question_type,
    location_format, special_category, consent_text_content_id, published_at
) VALUES
    ('de000002-0000-4000-8000-000000000001', 'de000001-0000-4000-8000-000000000001', 1, 'published',
        'de000005-0000-4000-8000-000000000001', 'number', NULL, false, NULL, NOW()),
    ('de000002-0000-4000-8000-000000000002', 'de000001-0000-4000-8000-000000000002', 1, 'published',
        'de000005-0000-4000-8000-000000000002', 'any_string', NULL, true,
        'de000005-0000-4000-8000-000000000006', NOW()),
    ('de000002-0000-4000-8000-000000000003', 'de000001-0000-4000-8000-000000000003', 1, 'published',
        'de000005-0000-4000-8000-000000000003', 'any_string', NULL, false, NULL, NOW()),
    ('de000002-0000-4000-8000-000000000004', 'de000001-0000-4000-8000-000000000004', 1, 'published',
        'de000005-0000-4000-8000-000000000004', 'location', 'postcode', false, NULL, NOW()),
    ('de000002-0000-4000-8000-000000000005', 'de000001-0000-4000-8000-000000000005', 1, 'published',
        'de000005-0000-4000-8000-000000000005', 'any_string', NULL, false, NULL, NOW());

-- Move the answers collected through the old slug-keyed model onto the default question versions.
-- Values use the shape each question type expects: numbers as JSON numbers, strings as JSON strings,
-- and postcodes as a location object.
INSERT INTO demographics_response (question_version_id, user_id, value, created_at, updated_at)
SELECT
    m.version_id,
    dr.user_id,
    CASE
        WHEN dr.question_slug = 'age' THEN to_jsonb(dr.value::NUMERIC)
        WHEN dr.question_slug = 'zipcode' THEN jsonb_build_object('postcode', dr.value)
        ELSE to_jsonb(dr.value)
    END,
    NOW(),
    NOW()
FROM legacy_demographics_response dr
JOIN (VALUES
    ('age', 'de000002-0000-4000-8000-000000000001'::UUID),
    ('ethnicity', 'de000002-0000-4000-8000-000000000002'::UUID),
    ('gender', 'de000002-0000-4000-8000-000000000003'::UUID),
    ('zipcode', 'de000002-0000-4000-8000-000000000004'::UUID),
    ('political_party', 'de000002-0000-4000-8000-000000000005'::UUID)
) AS m(slug, version_id) ON m.slug = dr.question_slug
WHERE dr.value IS NOT NULL AND (dr.question_slug <> 'age' OR dr.value ~ '^-?[0-9]+(\.[0-9]+)?$');

-- Turn each conversation's old demographics link into a question step at the end of its workflow.
-- Those steps ask the latest published version and are optional, as the old questions were.
DO $$
DECLARE
    link RECORD;
    name_content_id UUID;
    next_order INT;
BEGIN
    FOR link IN
        SELECT w.id AS workflow_id, m.question_id, m.display_name
        FROM legacy_conversation_demographics cd
        JOIN workflow w ON w.conversation_id = cd.conversation_id
        JOIN (VALUES
            ('age', 'de000001-0000-4000-8000-000000000001'::UUID, 'Age', 1),
            ('ethnicity', 'de000001-0000-4000-8000-000000000002'::UUID, 'Ethnicity', 2),
            ('gender', 'de000001-0000-4000-8000-000000000003'::UUID, 'Gender', 3),
            ('zipcode', 'de000001-0000-4000-8000-000000000004'::UUID, 'Postcode', 4),
            ('political_party', 'de000001-0000-4000-8000-000000000005'::UUID, 'Political affiliation', 5)
        ) AS m(slug, question_id, display_name, ordinal) ON m.slug = cd.question_slug
        ORDER BY cd.conversation_id, m.ordinal
    LOOP
        INSERT INTO text_content (primary_locale, format) VALUES ('en', 'plain')
        RETURNING id INTO name_content_id;

        INSERT INTO text_translation (content_id, locale, content, ai_generated, requires_validation)
        VALUES (name_content_id, 'en', link.display_name, false, false);

        SELECT COALESCE(MAX(step_order), -1) + 1 INTO next_order
        FROM workflow_step WHERE workflow_id = link.workflow_id;

        INSERT INTO workflow_step (
            workflow_id, name, step_order, activation_rule, tool_config, preview_tool_config,
            required, can_revisit, is_offline
        ) VALUES (
            link.workflow_id,
            name_content_id,
            next_order,
            to_jsonb('manual'::TEXT),
            jsonb_build_object(
                'type', 'demographicquestion',
                'question_id', link.question_id,
                'version', jsonb_build_object('type', 'latest')
            ),
            jsonb_build_object(
                'type', 'demographicquestion',
                'question_id', link.question_id,
                'version', jsonb_build_object('type', 'latest')
            ),
            false, false, false
        );
    END LOOP;
END $$;

DROP TABLE legacy_conversation_demographics;
DROP TABLE legacy_demographics_response;
DROP TABLE legacy_demographics_question;
DROP TYPE demographics_response_type;
