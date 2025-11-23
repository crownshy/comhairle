CREATE TABLE TEXT_CONTENT(
    id uuid PRIMARY KEY DEFAULT uuid_generate_v4(),
	primary_locale TEXT NOT NULL DEFAULT "en",
	format TEXT NOT NULL DEFAULT "plain"  CHECK (storage_type IN ('markdown', 'plain', 'rich')),
	conversation_id uuid REFERENCES conversation(id) ON DELETE CASCADE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()	
)-- Add migration script here

