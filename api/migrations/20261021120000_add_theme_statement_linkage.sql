-- Links statements to the themes they were classified under, and records where a
-- statement and a theme collection came from (which tool, and which part of that
-- tool's data) so provenance survives independent of the workflow step's tool config.

ALTER TABLE statement
    ADD COLUMN source_tool TEXT NOT NULL DEFAULT 'manual' CHECK (btrim(source_tool) <> ''),
    ADD COLUMN source_sub_id TEXT,
    ADD COLUMN theme_id UUID REFERENCES theme(id) ON DELETE SET NULL;

ALTER TABLE statement ALTER COLUMN source_tool DROP DEFAULT;

CREATE INDEX idx_statement_theme_id ON statement(theme_id);
CREATE INDEX idx_statement_source ON statement(workflow_step_id, source_tool, source_sub_id);

ALTER TABLE theme_collection
    ADD COLUMN origin_tool TEXT NOT NULL DEFAULT 'manual' CHECK (btrim(origin_tool) <> ''),
    ADD COLUMN origin_sub_id TEXT,
    ADD COLUMN requested_by_user_id UUID REFERENCES comhairle_user(id) ON DELETE SET NULL;

ALTER TABLE theme_collection ALTER COLUMN origin_tool DROP DEFAULT;
