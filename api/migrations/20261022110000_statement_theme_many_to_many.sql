-- A statement can match more than one theme (the sensemaker model is explicitly
-- told this), so the single `statement.theme_id` column can't represent it -
-- replace it with a join table.

CREATE TABLE statement_theme (
    statement_id UUID NOT NULL REFERENCES statement(id) ON DELETE CASCADE,
    theme_id UUID NOT NULL REFERENCES theme(id) ON DELETE CASCADE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    PRIMARY KEY (statement_id, theme_id)
);

CREATE INDEX idx_statement_theme_theme_id ON statement_theme(theme_id);

INSERT INTO statement_theme (statement_id, theme_id)
SELECT id, theme_id FROM statement WHERE theme_id IS NOT NULL;

DROP INDEX idx_statement_theme_id;
ALTER TABLE statement DROP COLUMN theme_id;
