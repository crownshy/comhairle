-- Convert conversation table to use translatable content
-- This migration handles the complete conversion in a single step

-- First, add temporary UUID columns to hold the new TextContent IDs
ALTER TABLE conversation ADD COLUMN new_title uuid;
ALTER TABLE conversation ADD COLUMN new_short_description uuid;
ALTER TABLE conversation ADD COLUMN new_description uuid;

-- Create TextContent and TextTranslation entries for existing conversations
DO $$
DECLARE
    conv_record RECORD;
    title_content_id uuid;
    short_desc_content_id uuid;
    desc_content_id uuid;
BEGIN
    FOR conv_record IN SELECT id, title, short_description, description, primary_locale FROM conversation LOOP
        
        -- Create TextContent and TextTranslation for title (title is always required)
        INSERT INTO TEXT_CONTENT (primary_locale, format)
        VALUES (COALESCE(conv_record.primary_locale, 'en'), 'plain')
        RETURNING id INTO title_content_id;
        
        INSERT INTO TEXT_TRANSLATION (content_id, locale, content, ai_generated, requires_validation)
        VALUES (title_content_id, COALESCE(conv_record.primary_locale, 'en'), COALESCE(conv_record.title, ''), false, false);
        
        UPDATE conversation SET new_title = title_content_id WHERE id = conv_record.id;
        
        -- Create TextContent and TextTranslation for short_description
        IF conv_record.short_description IS NOT NULL THEN
            INSERT INTO TEXT_CONTENT (primary_locale, format)
            VALUES (COALESCE(conv_record.primary_locale, 'en'), 'rich')
            RETURNING id INTO short_desc_content_id;
            
            INSERT INTO TEXT_TRANSLATION (content_id, locale, content, ai_generated, requires_validation)
            VALUES (short_desc_content_id, COALESCE(conv_record.primary_locale, 'en'), conv_record.short_description, false, false);
            
            UPDATE conversation SET new_short_description = short_desc_content_id WHERE id = conv_record.id;
        END IF;
        
        -- Create TextContent and TextTranslation for description
        IF conv_record.description IS NOT NULL THEN
            INSERT INTO TEXT_CONTENT (primary_locale, format)
            VALUES (COALESCE(conv_record.primary_locale, 'en'), 'rich')
            RETURNING id INTO desc_content_id;
            
            INSERT INTO TEXT_TRANSLATION (content_id, locale, content, ai_generated, requires_validation)
            VALUES (desc_content_id, COALESCE(conv_record.primary_locale, 'en'), conv_record.description, false, false);
            
            UPDATE conversation SET new_description = desc_content_id WHERE id = conv_record.id;
        END IF;
        
    END LOOP;
END $$;

-- Drop the old text columns and rename the new UUID columns
ALTER TABLE conversation DROP COLUMN title;
ALTER TABLE conversation DROP COLUMN short_description;
ALTER TABLE conversation DROP COLUMN description;

ALTER TABLE conversation RENAME COLUMN new_title TO title;
ALTER TABLE conversation RENAME COLUMN new_short_description TO short_description;
ALTER TABLE conversation RENAME COLUMN new_description TO description;

-- Set the title column as NOT NULL since it's required
ALTER TABLE conversation ALTER COLUMN title SET NOT NULL;