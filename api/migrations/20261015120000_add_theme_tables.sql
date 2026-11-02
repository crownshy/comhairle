-- A theme collection groups the themes found for a workflow step (e.g. themes an AI
-- sensemaking pass pulled out of a step's statements). It belongs to a workflow step,
-- and `context` records what the themes were derived from.

CREATE TABLE theme_collection (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    workflow_step_id UUID NOT NULL REFERENCES workflow_step(id) ON DELETE CASCADE,
    context TEXT NOT NULL CHECK (btrim(context) <> ''),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_theme_collection_workflow_step_id ON theme_collection(workflow_step_id);

CREATE TABLE theme (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    theme_collection_id UUID NOT NULL REFERENCES theme_collection(id) ON DELETE CASCADE,
    title TEXT NOT NULL CHECK (btrim(title) <> ''),
    description TEXT NOT NULL CHECK (btrim(description) <> ''),
    created_by_ai BOOLEAN NOT NULL DEFAULT FALSE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_theme_theme_collection_id ON theme(theme_collection_id);
