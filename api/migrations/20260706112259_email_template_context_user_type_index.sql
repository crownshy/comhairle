-- Drop existing search indexes and replace with unique indexes

DROP INDEX IF EXISTS idx_email_config_user_email_type;

CREATE UNIQUE INDEX idx_email_config_user_email_type
ON email_template_config(owner_id, email_type)
WHERE owner_id IS NOT NULL;

DROP INDEX IF EXISTS idx_email_config_organization_email_type;

CREATE UNIQUE INDEX idx_email_config_organization_email_type
ON email_template_config(organization_id, email_type)
WHERE organization_id IS NOT NULL;
