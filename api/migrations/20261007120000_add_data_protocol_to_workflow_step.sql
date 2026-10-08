-- Per-step text telling participants how their data is used. NULL shows the tool default.
ALTER TABLE workflow_step
ADD COLUMN data_protocol UUID;
