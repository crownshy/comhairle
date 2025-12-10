CREATE TABLE TEXT_CONTENT(
    id uuid PRIMARY KEY DEFAULT uuid_generate_v4(),
	primary_locale TEXT NOT NULL DEFAULT 'en',
	format TEXT NOT NULL DEFAULT 'plain'  CHECK (format IN ('markdown', 'plain', 'rich')),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()	
)-- Add migration script here

