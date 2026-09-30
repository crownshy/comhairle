-- Add free-form metadata column to conversation. Owners can store any
-- JSON object here (e.g. integration ids, internal flags). NOT NULL with
-- a default of '{}' so all reads see a real object and we never have to
-- handle a NULL case in application code.
ALTER TABLE conversation
    ADD COLUMN metadata JSONB NOT NULL DEFAULT '{}'::jsonb;
