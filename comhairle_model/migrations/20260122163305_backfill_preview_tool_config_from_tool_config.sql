-- Backfill preview_tool_config with tool_config for all existing workflow_step records
UPDATE workflow_step
SET preview_tool_config = tool_config
WHERE preview_tool_config IS NULL
  AND tool_config IS NOT NULL;
