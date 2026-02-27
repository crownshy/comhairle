DO $$
BEGIN
    -- Drop FK if it exists
    IF EXISTS (
        SELECT 1 FROM pg_constraint 
        WHERE conname = 'text_translation_content_id_fkey'
    ) THEN
        ALTER TABLE text_translation
        DROP CONSTRAINT text_translation_content_id_fkey;
    END IF;

    -- Re-add it with cascade
    ALTER TABLE text_translation
    ADD CONSTRAINT text_translation_content_id_fkey
    FOREIGN KEY (content_id)
    REFERENCES text_content (id)
    ON DELETE CASCADE;
END $$;
