-- Add is_seed flag to polis_statement_aux

ALTER TABLE polis_statement_aux
    ADD COLUMN is_seed BOOLEAN NOT NULL DEFAULT FALSE;
