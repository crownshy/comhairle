-- Create table for resource permissions
CREATE TABLE resource_permission (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    entity_type TEXT NOT NULL,
    entity_id UUID NOT NULL,
    resource_type TEXT NOT NULL,
    resource_id UUID NOT NULL,
    role TEXT NOT NULL,
    granted_by_entity_type TEXT NOT NULL,
    granted_by_entity_id UUID NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE UNIQUE INDEX resource_permission_entity_resource_unique ON resource_permission(entity_type, entity_id, resource_type, resource_id);

CREATE INDEX resource_permission_entity_index on resource_permission(entity_id);

CREATE INDEX resource_permission_entity_resource_type_index on resource_permission(entity_id, resource_type);

CREATE INDEX resource_permission_resource_index on resource_permission(resource_id);

CREATE INDEX resource_permission_granted_by_index on resource_permission(granted_by_entity_id);
