-- Migration: convert legacy inline prioritization text fields into
-- text_content/text_translation records, replacing inline strings with
-- text_content UUIDs inside tool_config / preview_tool_config JSONB.

-- Add temporary legacy backup columns, populated only for rows this
-- migration will touch, so an incident can be rolled back with a single
-- UPDATE without needing a proper down-migration.
ALTER TABLE workflow_step
    ADD COLUMN IF NOT EXISTS legacy_tool_config jsonb,
    ADD COLUMN IF NOT EXISTS legacy_preview_tool_config jsonb;

-- ---------------------------------------------------------------------
-- Insert a text_content + text_translation row, return new content id
-- (mirrors `new_translation_tx`).
-- ---------------------------------------------------------------------
CREATE OR REPLACE FUNCTION pg_temp.migrate_new_translation(
    p_locale text,
    p_content text,
    p_format text DEFAULT 'plain'
) RETURNS uuid AS $$
DECLARE
    v_content_id uuid;
BEGIN
    INSERT INTO text_content (primary_locale, format)
    VALUES (p_locale, p_format)
    RETURNING id INTO v_content_id;

    INSERT INTO text_translation (
        content_id, locale, content, ai_generated, requires_validation
    ) VALUES (
        v_content_id, p_locale, p_content, false, false
    );

    RETURN v_content_id;
END;
$$ LANGUAGE plpgsql;

-- ---------------------------------------------------------------------
-- Has this question already been migrated? (text/labels are uuids
-- rather than plain strings).
-- ---------------------------------------------------------------------
CREATE OR REPLACE FUNCTION pg_temp.migrate_is_question_migrated(p_question jsonb)
RETURNS boolean AS $$
DECLARE
    v_type jsonb := p_question->'type';
    v_uuid_re text := '^[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}$';
BEGIN
    IF NOT (p_question ? 'text') OR (p_question->>'text') !~* v_uuid_re THEN
        RETURN false;
    END IF;

    IF jsonb_typeof(v_type) = 'string' THEN
        RETURN v_type = '"text"';
    ELSIF v_type ? 'continuous' THEN
        RETURN (v_type->'continuous'->>'min_label') ~* v_uuid_re
           AND (v_type->'continuous'->>'max_label') ~* v_uuid_re;
    ELSIF v_type ? 'likert_scale' THEN
        RETURN NOT EXISTS (
            SELECT 1
            FROM jsonb_array_elements(v_type->'likert_scale'->'categories') AS cat
            WHERE (cat->>'label') !~* v_uuid_re
        );
    ELSE
        RETURN false;
    END IF;
END;
$$ LANGUAGE plpgsql;

CREATE OR REPLACE FUNCTION pg_temp.migrate_is_config_migrated(p_config jsonb)
RETURNS boolean AS $$
BEGIN
    IF p_config IS NULL THEN
        RETURN true;
    END IF;

    IF (p_config->>'type') IS DISTINCT FROM 'prioritization' THEN
        RETURN false;
    END IF;

    RETURN NOT EXISTS (
        SELECT 1 FROM jsonb_array_elements(COALESCE(p_config->'questions', '[]'::jsonb)) q
        WHERE NOT pg_temp.migrate_is_question_migrated(q)
    ) AND NOT EXISTS (
        SELECT 1 FROM jsonb_array_elements(COALESCE(p_config->'section_questions', '[]'::jsonb)) q
        WHERE NOT pg_temp.migrate_is_question_migrated(q)
    );
END;
$$ LANGUAGE plpgsql;

-- ---------------------------------------------------------------------
-- Migrate a single legacy question object into the new shape.
-- ---------------------------------------------------------------------
CREATE OR REPLACE FUNCTION pg_temp.migrate_question(p_question jsonb, p_locale text)
RETURNS jsonb AS $$
DECLARE
    v_text_tc_id uuid;
    v_type jsonb := p_question->'type';
    v_new_type jsonb;
    v_min_tc_id uuid;
    v_max_tc_id uuid;
    v_cat jsonb;
    v_label_tc_id uuid;
    v_cats jsonb := '[]'::jsonb;
BEGIN
    v_text_tc_id := pg_temp.migrate_new_translation(p_locale, p_question->>'text', 'plain');

    IF jsonb_typeof(v_type) = 'string' THEN
        v_new_type := '"text"'::jsonb;

    ELSIF v_type ? 'continuous' THEN
        v_min_tc_id := pg_temp.migrate_new_translation(p_locale, v_type->'continuous'->>'min_label', 'plain');
        v_max_tc_id := pg_temp.migrate_new_translation(p_locale, v_type->'continuous'->>'max_label', 'plain');

        v_new_type := jsonb_build_object(
            'continuous', jsonb_build_object(
                'sub_steps', v_type->'continuous'->'sub_steps',
                'min_value', v_type->'continuous'->'min_value',
                'max_value', v_type->'continuous'->'max_value',
                'min_label', to_jsonb(v_min_tc_id),
                'max_label', to_jsonb(v_max_tc_id)
            )
        );

    ELSIF v_type ? 'likert_scale' THEN
        FOR v_cat IN SELECT * FROM jsonb_array_elements(v_type->'likert_scale'->'categories')
        LOOP
            v_label_tc_id := pg_temp.migrate_new_translation(p_locale, v_cat->>'label', 'plain');
            v_cats := v_cats || jsonb_build_array(
                jsonb_build_object('value', v_cat->'value', 'label', to_jsonb(v_label_tc_id))
            );
        END LOOP;

        v_new_type := jsonb_build_object('likert_scale', jsonb_build_object('categories', v_cats));

    ELSE
        RAISE EXCEPTION 'Unknown legacy question type: %', v_type;
    END IF;

    RETURN jsonb_build_object(
        'id', p_question->'id',
        'text', to_jsonb(v_text_tc_id),
        'type', v_new_type
    );
END;
$$ LANGUAGE plpgsql;

-- ---------------------------------------------------------------------
-- Migrate a full legacy tool_config object.
-- ---------------------------------------------------------------------
CREATE OR REPLACE FUNCTION pg_temp.migrate_config(p_config jsonb, p_locale text)
RETURNS jsonb AS $$
DECLARE
    v_questions jsonb := '[]'::jsonb;
    v_section_questions jsonb := '[]'::jsonb;
    v_q jsonb;
BEGIN
    FOR v_q IN SELECT * FROM jsonb_array_elements(COALESCE(p_config->'questions', '[]'::jsonb))
    LOOP
        v_questions := v_questions || jsonb_build_array(pg_temp.migrate_question(v_q, p_locale));
    END LOOP;

    FOR v_q IN SELECT * FROM jsonb_array_elements(COALESCE(p_config->'section_questions', '[]'::jsonb))
    LOOP
        v_section_questions := v_section_questions || jsonb_build_array(pg_temp.migrate_question(v_q, p_locale));
    END LOOP;

    RETURN jsonb_build_object(
        'type', 'prioritization',
        'questions', v_questions,
        'section_questions', v_section_questions,
        'randomize_order', p_config->'randomize_order',
        'alignment_question_id', p_config->'alignment_question_id',
        'required_reviews', p_config->'required_reviews'
    );
END;
$$ LANGUAGE plpgsql;

-- ---------------------------------------------------------------------
-- Main migration loop
-- ---------------------------------------------------------------------
DO $$
DECLARE
    v_step RECORD;
    v_locale text;
    v_new_tool_config jsonb;
    v_new_preview_tool_config jsonb;
    v_tool_migrated boolean;
    v_backup_tool_config jsonb;
    v_backup_preview_tool_config jsonb;
BEGIN
    FOR v_step IN
        SELECT ws.id, ws.tool_config, ws.preview_tool_config, c.primary_locale
        FROM workflow_step ws
        JOIN workflow w ON w.id = ws.workflow_id
        JOIN conversation c ON c.id = w.conversation_id
        WHERE (ws.tool_config ->> 'type' = 'prioritization')
           OR (ws.preview_tool_config ->> 'type' = 'prioritization')
    LOOP
        v_locale := v_step.primary_locale;
        v_backup_tool_config := NULL;
        v_backup_preview_tool_config := NULL;

        IF pg_temp.migrate_is_config_migrated(v_step.preview_tool_config)
           AND (v_step.tool_config IS NULL OR pg_temp.migrate_is_config_migrated(v_step.tool_config))
        THEN
            RAISE NOTICE 'Step % already migrated', v_step.id;
            CONTINUE;
        END IF;

        IF v_step.tool_config IS NOT NULL AND NOT pg_temp.migrate_is_config_migrated(v_step.tool_config) THEN
            v_backup_tool_config := v_step.tool_config;
            v_new_tool_config := pg_temp.migrate_config(v_step.tool_config, v_locale);
            v_tool_migrated := true;
        ELSE
            v_new_tool_config := v_step.tool_config;
            v_tool_migrated := false;
        END IF;

        IF NOT pg_temp.migrate_is_config_migrated(v_step.preview_tool_config) THEN
            v_backup_preview_tool_config := v_step.preview_tool_config;
            v_new_preview_tool_config := pg_temp.migrate_config(v_step.preview_tool_config, v_locale);
        ELSE
            v_new_preview_tool_config := v_step.preview_tool_config;
        END IF;

        RAISE NOTICE 'Step %: tool_config migrated=% preview_tool_config_migrated=true',
            v_step.id, v_tool_migrated;

        UPDATE workflow_step
        SET tool_config = v_new_tool_config,
            preview_tool_config = v_new_preview_tool_config,
            legacy_tool_config = v_backup_tool_config,
            legacy_preview_tool_config = v_backup_preview_tool_config
        WHERE id = v_step.id;
    END LOOP;
END;
$$ LANGUAGE plpgsql;
