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

-- Legacy slug-keyed questions that hold answers or are linked to a conversation become system
-- questions with new UUIDs, so those answers and links carry over. Unused ones are dropped.
CREATE TABLE legacy_question_migration (
    slug TEXT PRIMARY KEY,
    question_id UUID NOT NULL DEFAULT uuid_generate_v4(),
    version_id UUID NOT NULL DEFAULT uuid_generate_v4(),
    display_name TEXT NOT NULL,
    question_text TEXT NOT NULL,
    question_type TEXT NOT NULL,
    location_format TEXT,
    consent_text TEXT,
    ordinal INT NOT NULL
);

INSERT INTO legacy_question_migration (
    slug, display_name, question_text, question_type, location_format, consent_text, ordinal
)
SELECT
    lq.slug,
    COALESCE(w.display_name, lq.display_name),
    COALESCE(w.question_text, lq.display_name),
    COALESCE(w.question_type, CASE WHEN lq.response_type::TEXT = 'number' THEN 'number' ELSE 'any_string' END),
    w.location_format,
    w.consent_text,
    COALESCE(w.ordinal, 100)
FROM legacy_demographics_question lq
LEFT JOIN (VALUES
    ('age', 'Age', 'What is your age?', 'number', NULL, NULL, 1),
    ('ethnicity', 'Ethnicity', 'What is your ethnicity?', 'any_string', NULL,
        'Ethnicity is special category data. We only ask for it with your consent, and we use it to understand who takes part.', 2),
    ('gender', 'Gender', 'What is your gender?', 'any_string', NULL, NULL, 3),
    ('zipcode', 'Postcode', 'What is your postcode?', 'location', 'postcode', NULL, 4),
    ('political_party', 'Political affiliation', 'What is your political affiliation?', 'any_string', NULL, NULL, 5)
) AS w(slug, display_name, question_text, question_type, location_format, consent_text, ordinal)
    ON w.slug = lq.slug
WHERE EXISTS (SELECT 1 FROM legacy_demographics_response dr WHERE dr.question_slug = lq.slug)
    OR EXISTS (SELECT 1 FROM legacy_conversation_demographics cd WHERE cd.question_slug = lq.slug);

DO $$
DECLARE
    q RECORD;
    question_text_id UUID;
    consent_text_id UUID;
    category_id UUID;
BEGIN
    FOR q IN SELECT * FROM legacy_question_migration ORDER BY ordinal, slug
    LOOP
        INSERT INTO text_content (primary_locale, format) VALUES ('en', 'plain')
        RETURNING id INTO question_text_id;
        INSERT INTO text_translation (content_id, locale, content, ai_generated, requires_validation)
        VALUES (question_text_id, 'en', q.question_text, false, false);

        consent_text_id := NULL;
        IF q.consent_text IS NOT NULL THEN
            INSERT INTO text_content (primary_locale, format) VALUES ('en', 'plain')
            RETURNING id INTO consent_text_id;
            INSERT INTO text_translation (content_id, locale, content, ai_generated, requires_validation)
            VALUES (consent_text_id, 'en', q.consent_text, false, false);
        END IF;

        -- The admin UI needs a category before a question can be republished.
        INSERT INTO demographics_tag (organization_id, name) VALUES (NULL, q.display_name)
        ON CONFLICT (name) WHERE organization_id IS NULL DO UPDATE SET name = EXCLUDED.name
        RETURNING id INTO category_id;

        INSERT INTO demographics_question (id, organization_id) VALUES (q.question_id, NULL);
        INSERT INTO demographics_question_tag (question_id, tag_id) VALUES (q.question_id, category_id);
        INSERT INTO demographics_question_version (
            id, question_id, version_number, status, text_content_id, question_type,
            location_format, special_category, consent_text_content_id, published_at
        ) VALUES (
            q.version_id, q.question_id, 1, 'published', question_text_id, q.question_type,
            q.location_format, consent_text_id IS NOT NULL, consent_text_id, NOW()
        );
    END LOOP;
END $$;

-- Values use the shape each question type expects: numbers as JSON numbers, strings as JSON strings,
-- and postcodes as a location object.
INSERT INTO demographics_response (question_version_id, user_id, value, created_at, updated_at)
SELECT
    m.version_id,
    dr.user_id,
    CASE m.question_type
        WHEN 'number' THEN to_jsonb(dr.value::NUMERIC)
        WHEN 'location' THEN jsonb_build_object('postcode', dr.value)
        ELSE to_jsonb(dr.value)
    END,
    NOW(),
    NOW()
FROM legacy_demographics_response dr
JOIN legacy_question_migration m ON m.slug = dr.question_slug
WHERE dr.value IS NOT NULL AND (m.question_type <> 'number' OR dr.value ~ '^-?[0-9]+(\.[0-9]+)?$');

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
        JOIN legacy_question_migration m ON m.slug = cd.question_slug
        ORDER BY cd.conversation_id, m.ordinal, m.slug
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

DROP TABLE legacy_question_migration;
DROP TABLE legacy_conversation_demographics;
DROP TABLE legacy_demographics_response;
DROP TABLE legacy_demographics_question;
DROP TYPE demographics_response_type;
