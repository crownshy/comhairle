-- Add ai_generated_summary column to thinking_space_summary table

ALTER TABLE thinking_space_summary
ADD COLUMN ai_generated_summary TEXT;
