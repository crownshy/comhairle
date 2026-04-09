-- Add show_thank_you_page_annon_instructions to conversation table
ALTER TABLE conversation
ADD COLUMN show_thank_you_page_annon_instructions BOOLEAN DEFAULT TRUE;
