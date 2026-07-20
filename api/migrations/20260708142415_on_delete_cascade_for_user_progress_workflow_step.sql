-- Drop existing foreign key constraint and replace with ON DELETE CASCADE added

ALTER TABLE user_progress
DROP CONSTRAINT user_progress_workflow_step_id_fkey;

ALTER TABLE user_progress
ADD CONSTRAINT user_progress_workflow_step_id_fkey
FOREIGN KEY (workflow_step_id)
REFERENCES workflow_step(id)
ON DELETE CASCADE;
