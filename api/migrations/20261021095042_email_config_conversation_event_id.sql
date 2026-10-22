-- Add conversation_id and event_id to email_template_config
-- Adjust indexes to allow more than one of each type depending on null status of new columns

ALTER TABLE email_template_config
ADD COLUMN conversation_id UUID REFERENCES conversation(id) ON DELETE CASCADE,
ADD COLUMN event_id UUID REFERENCES event(id) ON DELETE CASCADE,
ADD CONSTRAINT email_template_config_single_scope
    CHECK (conversation_id IS NULL OR event_id IS NULL),
ADD CONSTRAINT email_template_config_event_scope_type
    CHECK (event_id IS NULL OR email_type <> 'conversation_invite');


DROP INDEX IF EXISTS idx_email_config_user_email_type;
DROP INDEX IF EXISTS idx_email_config_organization_email_type;

CREATE UNIQUE INDEX idx_email_config_user_email_type
ON email_template_config(owner_id, email_type)
WHERE conversation_id IS NULL AND event_id IS NULL;

CREATE UNIQUE INDEX idx_email_config_conversation_email_type
ON email_template_config(conversation_id, email_type)
WHERE conversation_id IS NOT NULL;

CREATE UNIQUE INDEX idx_email_config_event_email_type
ON email_template_config(event_id, email_type)
WHERE event_id IS NOT NULL;
