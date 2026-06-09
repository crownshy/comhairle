-- Add request_user_share_permission column to workflow_step

ALTER TABLE workflow_step
ADD COLUMN request_user_share_permission BOOLEAN DEFAULT FALSE;
