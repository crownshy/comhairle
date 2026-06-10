-- Add email_template_config table

CREATE TABLE email_template_config (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    owner_id UUID REFERENCES comhairle_user(id), 
    organization_id UUID REFERENCES organization(id),
    email_type TEXT NOT NULL,
    slots JSONB NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
