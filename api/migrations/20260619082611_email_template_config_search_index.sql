-- Add search indexes to email_template_config

CREATE INDEX idx_email_config_user_email_type
ON email_template_config(owner_id, email_type)
WHERE owner_id IS NOT NULL;

CREATE INDEX idx_email_config_organization_email_type
ON email_template_config(organization_id, email_type)
WHERE organization_id IS NOT NULL;
