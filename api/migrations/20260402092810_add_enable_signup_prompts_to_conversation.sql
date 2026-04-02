-- Add enable_signup_prompts to conversation table
ALTER TABLE conversation
ADD COLUMN enable_signup_prompts BOOLEAN DEFAULT TRUE;
