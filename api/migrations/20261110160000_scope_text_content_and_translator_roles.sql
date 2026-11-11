-- Add conversation_id and organization_id columns to text_content, with scoping constraints.
ALTER TABLE text_content
    ADD COLUMN conversation_id UUID,
    ADD COLUMN organization_id UUID,
    ADD CONSTRAINT text_content_scope_exclusive
        CHECK (conversation_id IS NULL OR organization_id IS NULL),
    ADD CONSTRAINT text_content_conversation_fk
        FOREIGN KEY (conversation_id) REFERENCES conversation(id)
        ON DELETE CASCADE DEFERRABLE INITIALLY DEFERRED,
    ADD CONSTRAINT text_content_organization_fk
        FOREIGN KEY (organization_id) REFERENCES organization(id)
        ON DELETE CASCADE DEFERRABLE INITIALLY DEFERRED;

-- Add indexes for conversation_id and organization_id columns.
CREATE INDEX text_content_conversation_id_idx ON text_content(conversation_id);
CREATE INDEX text_content_organization_id_idx ON text_content(organization_id);

-- Create a temporary table to hold candidate scopes for text content.
CREATE TEMP TABLE text_content_scope_candidates (
    content_id UUID NOT NULL,
    resource_type TEXT NOT NULL,
    resource_id UUID,
    CHECK (
        (resource_type = 'system' AND resource_id IS NULL)
        OR (resource_type IN ('conversation', 'organization') AND resource_id IS NOT NULL)
    )
) ON COMMIT DROP;

-- Add conversation-scoped text content from conversation table.
INSERT INTO text_content_scope_candidates
SELECT content_id, 'conversation', conversation_id
FROM (
    SELECT title AS content_id, id AS conversation_id FROM conversation
    UNION ALL SELECT short_description, id FROM conversation
    UNION ALL SELECT description, id FROM conversation
    UNION ALL SELECT privacy_policy, id FROM conversation
    UNION ALL SELECT short_privacy_policy, id FROM conversation
    UNION ALL SELECT faqs, id FROM conversation
    UNION ALL SELECT thank_you_message, id FROM conversation
    UNION ALL SELECT call_to_action, id FROM conversation
) AS conversation_text
WHERE content_id IS NOT NULL;

-- Add conversation-scoped text content from workflow_step table.
INSERT INTO text_content_scope_candidates
SELECT content_id, 'conversation', COALESCE(workflow.conversation_id, event.conversation_id)
FROM (
    SELECT name AS content_id, workflow_id FROM workflow_step
    UNION ALL SELECT description, workflow_id FROM workflow_step
    UNION ALL SELECT data_protocol, workflow_id FROM workflow_step
) AS step_text
JOIN workflow ON workflow.id = step_text.workflow_id
LEFT JOIN event ON event.id = workflow.event_id
WHERE content_id IS NOT NULL;

-- Add conversation-scoped text content from event table.
INSERT INTO text_content_scope_candidates
SELECT content_id, 'conversation', conversation_id
FROM (
    SELECT name AS content_id, conversation_id FROM event
    UNION ALL SELECT description, conversation_id FROM event
) AS event_text
WHERE content_id IS NOT NULL;

-- Add conversation-scoped text content from report table.
INSERT INTO text_content_scope_candidates
SELECT content_id, 'conversation', conversation_id
FROM (
    SELECT summary AS content_id, conversation_id FROM report
    UNION ALL SELECT body, conversation_id FROM report
) AS report_text
WHERE content_id IS NOT NULL;

-- Add conversation-scoped text content from proposal_evaluation_proposal table.
INSERT INTO text_content_scope_candidates
SELECT proposal.title, 'conversation', COALESCE(workflow.conversation_id, event.conversation_id)
FROM proposal_evaluation_proposal AS proposal
JOIN workflow_step ON workflow_step.id = proposal.workflow_step_id
JOIN workflow ON workflow.id = workflow_step.workflow_id
LEFT JOIN event ON event.id = workflow.event_id
WHERE proposal.title IS NOT NULL;

-- Add conversation-scoped text content from proposal_evaluation_proposal_section table.
INSERT INTO text_content_scope_candidates
SELECT section.body, 'conversation', COALESCE(workflow.conversation_id, event.conversation_id)
FROM proposal_evaluation_proposal_section AS section
JOIN proposal_evaluation_proposal AS proposal ON proposal.id = section.proposal_id
JOIN workflow_step ON workflow_step.id = proposal.workflow_step_id
JOIN workflow ON workflow.id = workflow_step.workflow_id
LEFT JOIN event ON event.id = workflow.event_id
WHERE section.body IS NOT NULL;

-- Add organization-scoped text content from organization table.
INSERT INTO text_content_scope_candidates
SELECT content_id, 'organization', organization_id
FROM (
    SELECT description AS content_id, id AS organization_id FROM organization
    UNION ALL SELECT mission, id FROM organization
) AS organization_text
WHERE content_id IS NOT NULL;

-- Add system-scoped text content from region table.
INSERT INTO text_content_scope_candidates
SELECT content_id, 'system', NULL
FROM (
    SELECT name AS content_id FROM region
    UNION ALL SELECT description FROM region
) AS region_text
WHERE content_id IS NOT NULL;

-- Function to extract text content IDs from JSONB documents.
CREATE FUNCTION extract_text_content_ids(document JSONB)
RETURNS SETOF UUID
LANGUAGE SQL
STABLE
AS $$
    WITH RECURSIVE values_in_document(value) AS (
        SELECT document
        UNION ALL
        SELECT child.value
        FROM values_in_document
        CROSS JOIN LATERAL (
            SELECT value FROM jsonb_each(
                CASE WHEN jsonb_typeof(values_in_document.value) = 'object'
                    THEN values_in_document.value ELSE '{}'::JSONB END
            )
            UNION ALL
            SELECT value FROM jsonb_array_elements(
                CASE WHEN jsonb_typeof(values_in_document.value) = 'array'
                    THEN values_in_document.value ELSE '[]'::JSONB END
            )
        ) AS child
    )
    SELECT (value #>> '{}')::UUID
    FROM values_in_document
    WHERE jsonb_typeof(value) = 'string'
      AND value #>> '{}' ~* '^[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}$'
      AND EXISTS (
          SELECT 1 FROM text_content WHERE id = (value #>> '{}')::UUID
      );
$$;

-- Add conversation-scoped text content from JSONB documents in workflow_step table.
INSERT INTO text_content_scope_candidates
SELECT DISTINCT extracted.content_id, 'conversation',
       COALESCE(workflow.conversation_id, event.conversation_id)
FROM workflow_step
JOIN workflow ON workflow.id = workflow_step.workflow_id
LEFT JOIN event ON event.id = workflow.event_id
CROSS JOIN LATERAL extract_text_content_ids(
    jsonb_build_array(workflow_step.tool_config, workflow_step.preview_tool_config)
) AS extracted(content_id);

-- Add conversation-scoped text content from JSONB documents in event table.
INSERT INTO text_content_scope_candidates
SELECT DISTINCT extracted.content_id, 'conversation', event.conversation_id
FROM event
CROSS JOIN LATERAL extract_text_content_ids(event.agenda) AS extracted(content_id)
WHERE event.agenda IS NOT NULL;

-- Add conversation-scoped text content from JSONB documents in report table.
INSERT INTO text_content_scope_candidates
SELECT DISTINCT extracted.content_id, 'conversation', report.conversation_id
FROM report
CROSS JOIN LATERAL extract_text_content_ids(report.section_configs) AS extracted(content_id)
WHERE report.section_configs IS NOT NULL;

-- Add conversation-scoped text content from JSONB documents in email_template_config table.
INSERT INTO text_content_scope_candidates
SELECT DISTINCT extracted.content_id,
       CASE
           WHEN email_template_config.conversation_id IS NOT NULL OR email_event.conversation_id IS NOT NULL
               THEN 'conversation'
           WHEN email_template_config.organization_id IS NOT NULL THEN 'organization'
           ELSE 'system'
       END,
       COALESCE(email_template_config.conversation_id, email_event.conversation_id,
                email_template_config.organization_id)
FROM email_template_config
LEFT JOIN event AS email_event ON email_event.id = email_template_config.event_id
CROSS JOIN LATERAL extract_text_content_ids(email_template_config.slots) AS extracted(content_id);

-- Validate text content scope candidates before updating text_content table.
DO $$
DECLARE
    unclassified_count BIGINT;
    invalid_reference_count BIGINT;
    ambiguous_count BIGINT;
BEGIN
    -- Check for new text content scope candidates that don't correspond to any row in the text_content table.
    SELECT COUNT(*) INTO invalid_reference_count
    FROM text_content_scope_candidates AS candidate
    LEFT JOIN text_content ON text_content.id = candidate.content_id
    WHERE text_content.id IS NULL;
    IF invalid_reference_count > 0 THEN
        RAISE EXCEPTION 'Cannot scope translations: % resource references point to missing TextContent rows',
            invalid_reference_count;
    END IF;

    -- Check for content IDs that are referenced across multiple scopes.
    SELECT COUNT(*) INTO ambiguous_count
    FROM (
        SELECT content_id
        FROM text_content_scope_candidates
        GROUP BY content_id
        HAVING COUNT(DISTINCT (resource_type, resource_id)) > 1
    ) AS ambiguous;
    IF ambiguous_count > 0 THEN
        RAISE EXCEPTION 'Cannot scope translations: % TextContent rows are referenced across multiple scopes',
            ambiguous_count;
    END IF;

    -- Check for text content rows that have no corresponding scope candidate.
    SELECT COUNT(*) INTO unclassified_count
    FROM text_content
    LEFT JOIN text_content_scope_candidates AS candidate
        ON candidate.content_id = text_content.id
    WHERE candidate.content_id IS NULL;
    IF unclassified_count > 0 THEN
        RAISE EXCEPTION 'Cannot scope translations: % TextContent rows have no verifiable resource reference',
            unclassified_count;
    END IF;
END;
$$;

-- Update conversation / organization id for text content based on candidate scopes.
WITH resolved_scopes AS (
    SELECT DISTINCT ON (content_id) content_id, resource_type, resource_id
    FROM text_content_scope_candidates
)
UPDATE text_content
SET conversation_id = CASE WHEN resource_type = 'conversation' THEN resource_id END,
    organization_id = CASE WHEN resource_type = 'organization' THEN resource_id END
FROM resolved_scopes
WHERE text_content.id = resolved_scopes.content_id;

-- Enforce text content scope references via a trigger function.
CREATE FUNCTION enforce_text_content_scope_references()
RETURNS TRIGGER
LANGUAGE plpgsql
AS $$
DECLARE
    row_data JSONB := to_jsonb(NEW);
    scope_type TEXT;
    scope_id UUID;
    candidate_id UUID;
    field_name TEXT;
    json_field TEXT;
    candidate_fields TEXT[];
    json_fields TEXT[];
    workflow_owner RECORD;
    organization_id UUID;
    conversation_id UUID;
BEGIN
    CASE TG_TABLE_NAME
        WHEN 'conversation' THEN
            scope_type := 'conversation';
            scope_id := NEW.id;
            candidate_fields := ARRAY[
                'title', 'short_description', 'description', 'privacy_policy',
                'short_privacy_policy', 'faqs', 'thank_you_message', 'call_to_action'
            ];
        WHEN 'organization' THEN
            scope_type := 'organization';
            scope_id := NEW.id;
            candidate_fields := ARRAY['description', 'mission'];
        WHEN 'workflow_step' THEN
            SELECT COALESCE(workflow.conversation_id, event.conversation_id)
            INTO conversation_id
            FROM workflow
            LEFT JOIN event ON event.id = workflow.event_id
            WHERE workflow.id = NEW.workflow_id;
            scope_type := 'conversation';
            scope_id := conversation_id;
            candidate_fields := ARRAY['name', 'description', 'data_protocol'];
            json_fields := ARRAY['tool_config', 'preview_tool_config'];
        WHEN 'event' THEN
            scope_type := 'conversation';
            scope_id := NEW.conversation_id;
            candidate_fields := ARRAY['name', 'description'];
            json_fields := ARRAY['agenda'];
        WHEN 'report' THEN
            scope_type := 'conversation';
            scope_id := NEW.conversation_id;
            candidate_fields := ARRAY['summary', 'body'];
            json_fields := ARRAY['section_configs'];
        WHEN 'proposal_evaluation_proposal' THEN
            SELECT COALESCE(workflow.conversation_id, event.conversation_id)
            INTO conversation_id
            FROM workflow_step
            JOIN workflow ON workflow.id = workflow_step.workflow_id
            LEFT JOIN event ON event.id = workflow.event_id
            WHERE workflow_step.id = NEW.workflow_step_id;
            scope_type := 'conversation';
            scope_id := conversation_id;
            candidate_fields := ARRAY['title'];
        WHEN 'proposal_evaluation_proposal_section' THEN
            SELECT COALESCE(workflow.conversation_id, event.conversation_id)
            INTO conversation_id
            FROM proposal_evaluation_proposal AS proposal
            JOIN workflow_step ON workflow_step.id = proposal.workflow_step_id
            JOIN workflow ON workflow.id = workflow_step.workflow_id
            LEFT JOIN event ON event.id = workflow.event_id
            WHERE proposal.id = NEW.proposal_id;
            scope_type := 'conversation';
            scope_id := conversation_id;
            candidate_fields := ARRAY['body'];
        WHEN 'region' THEN
            scope_type := 'system';
            candidate_fields := ARRAY['name', 'description'];
        WHEN 'email_template_config' THEN
            SELECT event.conversation_id INTO conversation_id
            FROM event WHERE event.id = NEW.event_id;
            scope_id := COALESCE(NEW.conversation_id, conversation_id, NEW.organization_id);
            scope_type := CASE
                WHEN COALESCE(NEW.conversation_id, conversation_id) IS NOT NULL THEN 'conversation'
                WHEN NEW.organization_id IS NOT NULL THEN 'organization'
                ELSE 'system'
            END;
            json_fields := ARRAY['slots'];
        ELSE
            RAISE EXCEPTION 'TextContent scope validation is not configured for table %', TG_TABLE_NAME;
    END CASE;

    IF scope_type <> 'system' AND scope_id IS NULL THEN
        RAISE EXCEPTION 'Cannot determine TextContent scope for row in %', TG_TABLE_NAME;
    END IF;

    FOREACH field_name IN ARRAY COALESCE(candidate_fields, ARRAY[]::TEXT[]) LOOP
        IF row_data->>field_name IS NOT NULL THEN
            candidate_id := (row_data->>field_name)::UUID;
            PERFORM assert_text_content_scope(candidate_id, scope_type, scope_id);
        END IF;
    END LOOP;

    FOREACH json_field IN ARRAY COALESCE(json_fields, ARRAY[]::TEXT[]) LOOP
        FOR candidate_id IN
            SELECT DISTINCT content_id
            FROM extract_text_content_ids(row_data->json_field) AS ids(content_id)
        LOOP
            PERFORM assert_text_content_scope(candidate_id, scope_type, scope_id);
        END LOOP;
    END LOOP;

    RETURN NEW;
END;
$$;

-- Assert that text content is scoped correctly based on its expected type and ID.
CREATE FUNCTION assert_text_content_scope(content_id UUID, expected_type TEXT, expected_id UUID)
RETURNS VOID
LANGUAGE plpgsql
AS $$
DECLARE
    actual_conversation_id UUID;
    actual_organization_id UUID;
BEGIN
    SELECT conversation_id, organization_id
    INTO actual_conversation_id, actual_organization_id
    FROM text_content
    WHERE id = content_id
    FOR KEY SHARE;

    IF NOT FOUND THEN
        RAISE EXCEPTION 'TextContent % does not exist', content_id;
    END IF;

    IF expected_type = 'conversation' AND actual_conversation_id IS DISTINCT FROM expected_id
        OR expected_type = 'organization' AND actual_organization_id IS DISTINCT FROM expected_id
        OR expected_type = 'system' AND (actual_conversation_id IS NOT NULL OR actual_organization_id IS NOT NULL)
    THEN
        RAISE EXCEPTION 'TextContent % is not scoped to % %', content_id, expected_type, expected_id;
    END IF;
END;
$$;

-- Prevent deletion of text content that is still referenced by other entities.
CREATE FUNCTION prevent_referenced_text_content_deletion()
RETURNS TRIGGER
LANGUAGE plpgsql
AS $$
BEGIN
    -- The order of cascaded deletions is not guaranteed by postgres.
    -- So, we don't block deletions when referenced text content might be deleted as part of a cascade. 
    IF pg_trigger_depth() > 1 THEN
        RETURN OLD;
    END IF;

    IF EXISTS (
        SELECT 1 FROM conversation
        WHERE OLD.id IN (
            title, short_description, description, privacy_policy,
            short_privacy_policy, faqs, thank_you_message, call_to_action
        )
    ) OR EXISTS (
        SELECT 1 FROM organization WHERE OLD.id IN (description, mission)
    ) OR EXISTS (
        SELECT 1 FROM workflow_step
        WHERE OLD.id IN (name, description, data_protocol)
    ) OR EXISTS (
        SELECT 1 FROM event WHERE OLD.id IN (name, description)
    ) OR EXISTS (
        SELECT 1 FROM report WHERE OLD.id IN (summary, body)
    ) OR EXISTS (
        SELECT 1 FROM proposal_evaluation_proposal WHERE title = OLD.id
    ) OR EXISTS (
        SELECT 1 FROM proposal_evaluation_proposal_section WHERE body = OLD.id
    ) OR EXISTS (
        SELECT 1 FROM region WHERE OLD.id IN (name, description)
    ) OR EXISTS (
        SELECT 1
        FROM workflow_step
        CROSS JOIN LATERAL extract_text_content_ids(
            jsonb_build_array(tool_config, preview_tool_config)
        ) AS ids(content_id)
        WHERE ids.content_id = OLD.id
    ) OR EXISTS (
        SELECT 1
        FROM event
        CROSS JOIN LATERAL extract_text_content_ids(agenda) AS ids(content_id)
        WHERE ids.content_id = OLD.id
    ) OR EXISTS (
        SELECT 1
        FROM report
        CROSS JOIN LATERAL extract_text_content_ids(section_configs) AS ids(content_id)
        WHERE ids.content_id = OLD.id
    ) OR EXISTS (
        SELECT 1
        FROM email_template_config
        CROSS JOIN LATERAL extract_text_content_ids(slots) AS ids(content_id)
        WHERE ids.content_id = OLD.id
    ) THEN
        RAISE EXCEPTION 'Cannot delete referenced TextContent %', OLD.id;
    END IF;

    RETURN OLD;
END;
$$;

-- Prevent changing the scope of existing text content.
CREATE FUNCTION prevent_text_content_scope_change()
RETURNS TRIGGER
LANGUAGE plpgsql
AS $$
BEGIN
    IF NEW.conversation_id IS DISTINCT FROM OLD.conversation_id
        OR NEW.organization_id IS DISTINCT FROM OLD.organization_id
    THEN
        RAISE EXCEPTION 'TextContent scope is immutable';
    END IF;
    RETURN NEW;
END;
$$;

-- Prevent change of scope or deletion of referenced text content via a trigger.
CREATE TRIGGER text_content_scope_immutable
BEFORE UPDATE OF conversation_id, organization_id ON text_content
FOR EACH ROW EXECUTE FUNCTION prevent_text_content_scope_change();
CREATE TRIGGER text_content_referenced_delete_guard
BEFORE DELETE ON text_content
FOR EACH ROW EXECUTE FUNCTION prevent_referenced_text_content_deletion();

-- Enforce text content scope references on other tables via triggers.
CREATE TRIGGER text_content_scope_reference_check
BEFORE INSERT OR UPDATE ON conversation
FOR EACH ROW EXECUTE FUNCTION enforce_text_content_scope_references();
CREATE TRIGGER text_content_scope_reference_check
BEFORE INSERT OR UPDATE ON organization
FOR EACH ROW EXECUTE FUNCTION enforce_text_content_scope_references();
CREATE TRIGGER text_content_scope_reference_check
BEFORE INSERT OR UPDATE ON workflow_step
FOR EACH ROW EXECUTE FUNCTION enforce_text_content_scope_references();
CREATE TRIGGER text_content_scope_reference_check
BEFORE INSERT OR UPDATE ON event
FOR EACH ROW EXECUTE FUNCTION enforce_text_content_scope_references();
CREATE TRIGGER text_content_scope_reference_check
BEFORE INSERT OR UPDATE ON report
FOR EACH ROW EXECUTE FUNCTION enforce_text_content_scope_references();
CREATE TRIGGER text_content_scope_reference_check
BEFORE INSERT OR UPDATE ON proposal_evaluation_proposal
FOR EACH ROW EXECUTE FUNCTION enforce_text_content_scope_references();
CREATE TRIGGER text_content_scope_reference_check
BEFORE INSERT OR UPDATE ON proposal_evaluation_proposal_section
FOR EACH ROW EXECUTE FUNCTION enforce_text_content_scope_references();
CREATE TRIGGER text_content_scope_reference_check
BEFORE INSERT OR UPDATE ON region
FOR EACH ROW EXECUTE FUNCTION enforce_text_content_scope_references();
CREATE TRIGGER text_content_scope_reference_check
BEFORE INSERT OR UPDATE ON email_template_config
FOR EACH ROW EXECUTE FUNCTION enforce_text_content_scope_references();

-- Remove translator role permissions from existing system users and groups.
DELETE FROM system_user_permissions WHERE role_name = 'translator';
DELETE FROM system_group_permissions WHERE role_name = 'translator';
