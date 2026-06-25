-- Create recruitment_target table to store demographic recruitment goals for workflows.
-- Each row records the target number of participants for a given metric/bucket combination
-- (e.g. metric = 'age', bucket = '18-24', target_count = 30).
CREATE TABLE recruitment_target (
    id uuid PRIMARY KEY DEFAULT uuid_generate_v4(),
    workflow_id uuid NOT NULL REFERENCES workflow(id) ON DELETE CASCADE,
    metric TEXT NOT NULL,
    bucket TEXT NOT NULL,
    target_count INTEGER NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Only one target row per (workflow, metric, bucket) combination
CREATE UNIQUE INDEX recruitment_target_workflow_metric_bucket_unique
    ON recruitment_target(workflow_id, metric, bucket);

-- Index for listing all targets for a workflow
CREATE INDEX recruitment_target_workflow_id_index ON recruitment_target(workflow_id);
