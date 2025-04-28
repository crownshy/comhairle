CREATE TABLE FEEDBACK(
    id Uuid DEFAULT uuid_generate_v4(),
    created_by Uuid REFERENCES comhairle_user,
    conversation_id Uuid REFERENCES conversation,
    content Text,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
