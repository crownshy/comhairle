-- Add migration script here
CREATE TABLE conversation(
    id uuid PRIMARY KEY DEFAULT uuid_generate_v4(),
    title Text NOT NULL,
    short_description Text,
    description Text,
    video_url Text, 
    image_url Text NOT NULL, 
    tags  Text[],
    is_public  boolean DEFAULT false,
    is_invite_only boolean DEFAULT false,
    is_complete boolean DEFAULT false,
    lauch_date TIMESTAMPTZ,
    close_date TIMESTAMPTZ, 
    slug Text,
    owner_id uuid REFERENCES comhairle_user,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE UNIQUE INDEX conversation_slug_index
ON conversation(slug);
