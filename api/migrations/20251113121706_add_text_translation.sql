-- Add migration script here
CREATE TABLE TEXT_TRANSLATION(
    id uuid PRIMARY KEY DEFAULT uuid_generate_v4(),
	content_id uuid References TEXT_CONTENT NOT NULL, 
	locale TEXT NOT NULL,
	content TEXT NOT NULL DEFAULT '',
	ai_generated BOOLEAN DEFAULT FALSE,
	requires_validation BOOLEAN DEFAULT FALSE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()	
)-- Add migration script here

