-- Pinned themes are carried forward as "existing themes" into the next generation
-- run for their collection, so the model reuses them instead of inventing
-- duplicates. Manually created themes (created_by_ai = false) are always carried
-- forward regardless of this flag.

ALTER TABLE theme ADD COLUMN pinned BOOLEAN NOT NULL DEFAULT FALSE;
