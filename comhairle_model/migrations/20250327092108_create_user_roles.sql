-- Add migration script here

CREATE TABLE resource_role (
    resource_kind TEXT NOT NULL,
    resource_id uuid NOT NULL,  -- WARNING: referential integrity not enforced
    resource_role TEXT NOT NULL,  -- NOTE: 'role' has a special meaning in SQL
    user_id uuid NOT NULL REFERENCES comhairle_user(id) ON DELETE CASCADE,
    PRIMARY KEY (resource_kind, resource_id, resource_role, user_id)
);
CREATE INDEX resource_role_by_user_and_role_idx ON resource_role (
    user_id,
    resource_role
);