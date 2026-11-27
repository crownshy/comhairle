CREATE TABLE region_area_pipeline (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    draft_definition JSONB NOT NULL,
    active_definition JSONB,
    status TEXT NOT NULL DEFAULT 'draft'
        CHECK (status IN ('draft', 'active', 'staged', 'deactivated')),
    generation BIGINT NOT NULL DEFAULT 0,
    run_status TEXT NOT NULL DEFAULT 'unscheduled'
        CHECK (run_status IN ('unscheduled', 'waiting', 'running', 'retry_pending')),
    next_run_at TIMESTAMPTZ,
    lease_id UUID,
    lease_expires_at TIMESTAMPTZ,
    run_started_at TIMESTAMPTZ,
    last_success_at TIMESTAMPTZ,
    last_error TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    CHECK (
        (status = 'draft' AND active_definition IS NULL) OR
        (status = 'active' AND active_definition = draft_definition) OR
        (status = 'staged' AND active_definition <> draft_definition) OR
        (status = 'deactivated' AND active_definition IS NOT NULL)
    ),
    CHECK ((status IN ('active', 'staged')) = (run_status <> 'unscheduled')),
    CHECK ((run_status = 'unscheduled') = (next_run_at IS NULL)),
    CHECK ((run_status = 'running') = (lease_id IS NOT NULL)),
    CHECK ((lease_id IS NULL) = (lease_expires_at IS NULL)),
    CHECK ((lease_id IS NULL) = (run_started_at IS NULL)),
    CHECK (run_status <> 'retry_pending' OR last_error IS NOT NULL)
);

CREATE INDEX region_area_pipeline_due_index
ON region_area_pipeline (next_run_at)
WHERE status IN ('active', 'staged');

CREATE TABLE region_area_pipeline_feature (
    pipeline_id UUID NOT NULL REFERENCES region_area_pipeline(id),
    external_id TEXT NOT NULL CHECK (external_id <> ''),
    region_area_id UUID NOT NULL UNIQUE REFERENCES region_area(id),
    region_id UUID NOT NULL UNIQUE REFERENCES region(id),
    managed_tags TEXT[] NOT NULL DEFAULT '{}',
    manual_tags TEXT[] NOT NULL DEFAULT '{}',
    status TEXT NOT NULL DEFAULT 'live' CHECK (status IN ('live', 'retired')),
    retired_at TIMESTAMPTZ,
    PRIMARY KEY (pipeline_id, external_id),
    CHECK ((status = 'retired') = (retired_at IS NOT NULL))
);

CREATE TABLE region_area_pipeline_snapshot (
    pipeline_id UUID NOT NULL REFERENCES region_area_pipeline(id),
    kind TEXT NOT NULL CHECK (kind IN ('preview', 'successful')),
    definition JSONB NOT NULL,
    snapshot BYTEA NOT NULL CHECK (octet_length(snapshot) <= 67108864),
    preview_generation BIGINT,
    state_hash TEXT,
    summary JSONB NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    PRIMARY KEY (pipeline_id, kind),
    CHECK ((kind = 'preview') = (preview_generation IS NOT NULL AND state_hash IS NOT NULL))
);
