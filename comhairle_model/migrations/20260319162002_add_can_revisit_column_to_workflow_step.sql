-- Add can_revisit column to workflow step table
ALTER TABLE workflow_step
ADD COLUMN can_revisit BOOLEAN DEFAULT false;
