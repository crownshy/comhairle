-- Add lineage pointer for derived (split / reworded) statements.
-- A derived statement (is_seed = FALSE, original_statement_id set) points at the
-- participant statement it was split or reworded from. Self-referential; ON DELETE
-- SET NULL so removing an original never cascades away its replacements.

ALTER TABLE polis_statement_aux
    ADD COLUMN original_statement_id UUID REFERENCES polis_statement_aux(id) ON DELETE SET NULL;
