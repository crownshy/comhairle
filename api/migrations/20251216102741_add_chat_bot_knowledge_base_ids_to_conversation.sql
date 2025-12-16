-- Add chat_bot_id and knowledge_base_id column to conversation table

ALTER TABLE conversation
ADD COLUMN knowledge_base_id VARCHAR(50),
ADD COLUMN chat_bot_id VARCHAR(50),
ADD COLUMN enable_qa_chat_bot BOOLEAN NOT NULL DEFAULT FALSE;
