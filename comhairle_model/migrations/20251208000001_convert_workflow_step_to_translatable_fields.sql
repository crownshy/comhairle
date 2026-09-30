-- Convert workflow_step table to use translatable content
-- This migration handles the complete conversion in a single step

-- First, add temporary UUID columns to hold the new TextContent IDs
ALTER TABLE workflow_step ADD COLUMN new_name uuid;
ALTER TABLE workflow_step ADD COLUMN new_description uuid;

-- Create TextContent and TextTranslation entries for existing workflow_steps
DO $$
DECLARE
    step_record RECORD;
    name_content_id uuid;
    desc_content_id uuid;
BEGIN
    FOR step_record IN SELECT id, name, description FROM workflow_step LOOP
        
        -- Create TextContent and TextTranslation for name (name is always required)
        INSERT INTO TEXT_CONTENT (primary_locale, format)
        VALUES ('en', 'plain')
        RETURNING id INTO name_content_id;
        
        INSERT INTO TEXT_TRANSLATION (content_id, locale, content, ai_generated, requires_validation)
        VALUES (name_content_id, 'en', COALESCE(step_record.name, ''), false, false);
        
        UPDATE workflow_step SET new_name = name_content_id WHERE id = step_record.id;
        
        -- Create TextContent and TextTranslation for description
        IF step_record.description IS NOT NULL THEN
            INSERT INTO TEXT_CONTENT (primary_locale, format)
            VALUES ('en', 'rich')
            RETURNING id INTO desc_content_id;
            
            INSERT INTO TEXT_TRANSLATION (content_id, locale, content, ai_generated, requires_validation)
            VALUES (desc_content_id, 'en', step_record.description, false, false);
            
            UPDATE workflow_step SET new_description = desc_content_id WHERE id = step_record.id;
        END IF;
        
    END LOOP;
END $$;

-- Drop the old text columns and rename the new UUID columns
ALTER TABLE workflow_step DROP COLUMN name;
ALTER TABLE workflow_step DROP COLUMN description;

ALTER TABLE workflow_step RENAME COLUMN new_name TO name;
ALTER TABLE workflow_step RENAME COLUMN new_description TO description;

-- Set the name column as NOT NULL since it's required
ALTER TABLE workflow_step ALTER COLUMN name SET NOT NULL;