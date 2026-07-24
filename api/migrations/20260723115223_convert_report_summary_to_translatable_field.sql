-- Convert report summary to use translatable content
-- This migration handles the complete conversation in a single step

-- First, add a temporary UUID column to hold the new TextContent ID
ALTER TABLE report ADD COLUMN new_summary uuid;

-- Create TextContent and TextTranslation entries for existing reports
DO $$
DECLARE
    report_record RECORD;
    summary_content_id uuid;
BEGIN
    FOR report_record IN SELECT id, summary FROM report LOOP

        -- Create TextContent and TextTranslation for summary
        IF report_record.summary IS NOT NULL THEN
            INSERT INTO TEXT_CONTENT (primary_locale, format)
            VALUES ('en', 'rich')
            RETURNING id INTO summary_content_id;

            INSERT INTO TEXT_TRANSLATION (content_id, locale, content, ai_generated, requires_validation)
            VALUES (summary_content_id, 'en', COALESCE(report_record.summary, ''), false, false);

            UPDATE report SET new_summary = summary_content_id WHERE id = report_record.id;
        END IF;

    END LOOP;
END $$;

-- Drop the old text column and rename the new UUID column
ALTER TABLE report DROP COLUMN summary;

ALTER TABLE report RENAME COLUMN new_summary TO summary;
