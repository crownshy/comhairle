CREATE TABLE INVITE_RESPONSE(
    id uuid PRIMARY KEY DEFAULT uuid_generate_v4(),
	invite_id uuid REFERENCES invite(id) ON DELETE CASCADE,
	user_id uuid REFERENCES comhairle_user,
	response TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE UNIQUE INDEX joint_invite_user_index on INVITE_RESPONSE(invite_id,user_id);
