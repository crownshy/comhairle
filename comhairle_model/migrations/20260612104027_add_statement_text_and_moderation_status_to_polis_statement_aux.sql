-- Add statement_text and moderation_status to polis_statement_aux

ALTER TABLE polis_statement_aux
    ADD COLUMN statement_text TEXT NOT NULL,
    ADD COLUMN moderation_status TEXT NOT NULL DEFAULT 'pending'
        CHECK (moderation_status IN ('accepted', 'rejected', 'pending'));
