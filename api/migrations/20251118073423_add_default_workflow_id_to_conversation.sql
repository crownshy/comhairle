-- Add migration script here
-- Add default_workflow_id field to conversation table

ALTER TABLE conversation 
ADD COLUMN default_workflow_id UUID REFERENCES workflow(id);