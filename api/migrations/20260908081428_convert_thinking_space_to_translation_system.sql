-- Convert legacy inline thinking-space text fields into
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
CREATE OR REPLACE FUNCTION pg_temp.ts_new_translation(
    p_locale text,
    p_content text,
    p_format text DEFAULT 'plain'
) RETURNS uuid AS $$
DECLARE
    v_content_id uuid;
    v_translation RECORD;
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

-- ------------------------------------------------------------------------------
-- Has this config already been migrated? (checks that "topic" is a UUID string).
-- ------------------------------------------------------------------------------
CREATE OR REPLACE FUNCTION pg_temp.ts_is_config_migrated(p_config jsonb)
RETURNS boolean AS $$
DECLARE
    v_uuid_re text := '^[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}$';
BEGIN
    IF p_config IS NULL THEN
        RETURN true;
    END IF;

    RETURN (p_config ? 'topic') AND (p_config->>'topic') ~* v_uuid_re;
END;
$$ LANGUAGE plpgsql;

-- ---------------------------------------------------------------------
-- Migrate a full legacy thinking-space tool_config object.
-- ---------------------------------------------------------------------
CREATE OR REPLACE FUNCTION pg_temp.ts_migrate_config(p_config jsonb, p_locale text)
RETURNS jsonb AS $$
DECLARE
    v_topic_tc_id uuid;
    v_question jsonb;
    v_text_tc_id uuid;
    v_intent_tc_id uuid;
    v_root_questions jsonb := '[]'::jsonb;
BEGIN
    v_topic_tc_id := pg_temp.ts_new_translation(p_locale, p_config->>'topic', 'plain');

    FOR v_question IN
        SELECT * FROM jsonb_array_elements(COALESCE(p_config->'root_questions', '[]'::jsonb))
    LOOP
        v_text_tc_id := pg_temp.ts_new_translation(p_locale, v_question->>'text', 'plain');
        v_intent_tc_id := pg_temp.ts_new_translation(p_locale, v_question->>'intent', 'plain');

        v_root_questions := v_root_questions || jsonb_build_array(
            jsonb_build_object(
                'id', v_question->'id',
                'text', to_jsonb(v_text_tc_id),
                'intent', to_jsonb(v_intent_tc_id)
            )
        );
    END LOOP;

    RETURN jsonb_build_object(
        'type', 'thinkingspace',
        'topic', to_jsonb(v_topic_tc_id),
        'root_questions', v_root_questions,
        'follow_up_rounds_count', p_config->'follow_up_rounds_count'
    );
END;
$$ LANGUAGE plpgsql;

-- ---------------------------------------------------------------------
-- Main migration loop.
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
        WHERE (ws.tool_config ->> 'type' = 'thinkingspace')
           OR (ws.preview_tool_config ->> 'type' = 'thinkingspace')
    LOOP
        v_locale := v_step.primary_locale;
        v_backup_tool_config := NULL;
        v_backup_preview_tool_config := NULL;

        IF pg_temp.ts_is_config_migrated(v_step.preview_tool_config)
           AND (v_step.tool_config IS NULL OR pg_temp.ts_is_config_migrated(v_step.tool_config))
        THEN
            RAISE NOTICE 'Step % already migrated', v_step.id;
            CONTINUE;
        END IF;

        IF v_step.tool_config IS NOT NULL AND NOT pg_temp.ts_is_config_migrated(v_step.tool_config) THEN
            v_backup_tool_config := v_step.tool_config;
            v_new_tool_config := pg_temp.ts_migrate_config(v_step.tool_config, v_locale);
            v_tool_migrated := true;
        ELSE
            v_new_tool_config := v_step.tool_config;
            v_tool_migrated := false;
        END IF;

        IF NOT pg_temp.ts_is_config_migrated(v_step.preview_tool_config) THEN
            v_backup_preview_tool_config := v_step.preview_tool_config;
            v_new_preview_tool_config := pg_temp.ts_migrate_config(v_step.preview_tool_config, v_locale);
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
