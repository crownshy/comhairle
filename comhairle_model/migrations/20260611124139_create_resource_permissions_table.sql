CREATE TABLE resource_permissions (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),

    -- Exclusive arc relationship is enfoced by check constraint below.
    -- NOTE: Not open for extension. Additional ID fields will require a new constraint.
    user_id         UUID REFERENCES comhairle_user(id) ON DELETE CASCADE,
    organization_id UUID REFERENCES organization(id)   ON DELETE CASCADE,

    resource_id   UUID          NOT NULL,
    resource_type VARCHAR(100)  NOT NULL,

    role_name VARCHAR(50) NOT NULL,

    granted_by  UUID REFERENCES comhairle_user(id) ON DELETE SET NULL,
    grant_reason TEXT         NOT NULL,
    granted_at  TIMESTAMPTZ  NOT NULL DEFAULT NOW(),

    CONSTRAINT chk_grantee_exclusive CHECK (
        (user_id IS NOT NULL AND organization_id IS NULL) OR
        (user_id IS NULL    AND organization_id IS NOT NULL)
    )
);

CREATE UNIQUE INDEX idx_unique_user_resource_role
    ON resource_permissions (user_id, resource_id, resource_type, role_name)
    WHERE user_id IS NOT NULL;

CREATE UNIQUE INDEX idx_unique_org_resource_role
    ON resource_permissions (organization_id, resource_id, resource_type, role_name)
    WHERE organization_id IS NOT NULL;

CREATE INDEX idx_resource_lookup
    ON resource_permissions (resource_type, resource_id);

CREATE INDEX idx_permissions_keyset
    ON resource_permissions (granted_at DESC, id DESC);

CREATE INDEX idx_permissions_resource_keyset
    ON resource_permissions (resource_type, resource_id, granted_at DESC, id DESC);
