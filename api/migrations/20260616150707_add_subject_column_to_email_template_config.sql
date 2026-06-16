-- Add subject column to email_template_config

ALTER TABLE email_template_config ADD COLUMN subject TEXT;
