CREATE TABLE REPORT(
    id uuid PRIMARY KEY DEFAULT uuid_generate_v4(),
    is_public boolean DEFAULT false,
    conversation_id uuid REFERENCES conversation,
    summary Text,
    section_configs JSONB,    
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
)-- Add migration script here
