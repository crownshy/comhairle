-- Pending access for an email recipient or an organization awaiting administrator acceptance.
CREATE TABLE pending_role_assignment (
    id uuid PRIMARY KEY DEFAULT uuid_generate_v4 (),
    email text,
    organization_id uuid REFERENCES organization (id) ON DELETE CASCADE,
    resource_type text NOT NULL CHECK (resource_type IN ('system', 'conversation', 'organization')),
    resource_id uuid NOT NULL,
    role_names text[] NOT NULL CHECK (cardinality(role_names) > 0),
    grant_reason text NOT NULL,
    invited_by uuid REFERENCES comhairle_user (id) ON DELETE SET NULL,
    created_at timestamptz NOT NULL DEFAULT now(),
    expires_at timestamptz NOT NULL DEFAULT now() + interval '14 days',
    CHECK ((email IS NOT NULL) <> (organization_id IS NOT NULL)),
    CHECK (organization_id IS NULL OR resource_type = 'conversation'),
    UNIQUE (email, resource_type, resource_id),
    UNIQUE (organization_id, resource_type, resource_id)
);
