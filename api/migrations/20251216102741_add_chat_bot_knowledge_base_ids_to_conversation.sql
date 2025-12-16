-- Add chat_bot_id and knowledge_base_id column to conversation table

ALTER TABLE conversation
ADD COLUMN knowledge_base_id UUID,
ADD COLUMN chat_bot_id UUID;
