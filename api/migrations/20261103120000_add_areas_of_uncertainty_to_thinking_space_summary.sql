-- Add areas_of_uncertainty column to thinking_space_summary table

ALTER TABLE thinking_space_summary
ADD COLUMN areas_of_uncertainty TEXT;
