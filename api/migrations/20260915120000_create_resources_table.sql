-- Central resource registry: conversation/organization ids are foreign keys
-- into this table, so a resources row must exist before the owning object
-- can be created (see api/src/models/resources.rs).
CREATE TABLE resources (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    owner_id UUID REFERENCES comhairle_user(id) ON DELETE SET NULL,
    -- Not currently used, but good for future-proofing and clarity of the resource's type.
    resource_type TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    -- ID is already unique (PK), this is added to ensure that foreign key references to (id, owner_id) can be enforced without postgres throwing errors.
    CONSTRAINT resources_id_owner_id_key UNIQUE (id, owner_id)
);

-- Reassigns `old_id` to a freshly generated UUID on `table_name`, propagating
-- the change to every foreign key column across the schema that references
-- it, so the id can be registered in `resources` without colliding with one
-- already claimed by another table.
CREATE OR REPLACE FUNCTION reassign_resource_id(table_name regclass, old_id UUID)
RETURNS UUID AS $$
DECLARE
    new_id UUID := uuid_generate_v4();
    fk RECORD;
BEGIN
    -- While a collision is VERY unlikely, Murphy will have his way, so we check anyway.
    WHILE EXISTS (SELECT 1 FROM resources WHERE id = new_id) LOOP
        new_id := uuid_generate_v4();
    END LOOP;

    EXECUTE format('UPDATE %s SET id = $1 WHERE id = $2', table_name) USING new_id, old_id;

    FOR fk IN
        SELECT c.conrelid::regclass AS referencing_table,
               a.attname AS referencing_column
        FROM pg_constraint c
        JOIN pg_attribute a
            ON a.attrelid = c.conrelid AND a.attnum = c.conkey[1]
        WHERE c.contype = 'f'
          AND c.confrelid = table_name
          AND array_length(c.conkey, 1) = 1
    LOOP
        EXECUTE format(
            'UPDATE %s SET %I = $1 WHERE %I = $2',
            fk.referencing_table, fk.referencing_column, fk.referencing_column
        ) USING new_id, old_id;
    END LOOP;

    RETURN new_id;
END;
$$ LANGUAGE plpgsql;

-- Backfill: register every existing conversation/organization row, regenerating
-- (and propagating) the id on the astronomically unlikely event of a collision.
DO $$
DECLARE
    rec RECORD;
    assigned_id UUID;
BEGIN
    FOR rec IN SELECT id, owner_id, created_at FROM conversation LOOP
        assigned_id := rec.id;
        IF EXISTS (SELECT 1 FROM resources WHERE id = assigned_id) THEN
            assigned_id := reassign_resource_id('conversation', assigned_id);
        END IF;
        INSERT INTO resources (id, owner_id, resource_type, created_at) VALUES (assigned_id, rec.owner_id, 'conversation', rec.created_at);
    END LOOP;

    FOR rec IN SELECT id, created_at FROM organization LOOP
        assigned_id := rec.id;
        IF EXISTS (SELECT 1 FROM resources WHERE id = assigned_id) THEN
            assigned_id := reassign_resource_id('organization', assigned_id);
        END IF;
        INSERT INTO resources (id, resource_type, created_at) VALUES (assigned_id, 'organization', rec.created_at);
    END LOOP;
END $$;

DROP FUNCTION reassign_resource_id(regclass, UUID);

-- Fixed system resource (SYSTEM_RESOURCE_ID sentinel).
INSERT INTO resources (id, resource_type) VALUES ('00000000-0000-0000-0000-000000000000', 'system');

-- Backfill the owner_id for existing conversations.
UPDATE resources r
SET owner_id = c.owner_id
FROM conversation c
WHERE r.id = c.id;

-- New rows must be created via the resources model first; the old column
-- default is no longer sufficient since it wouldn't register a resources row.
ALTER TABLE conversation
    ALTER COLUMN id DROP DEFAULT,
    -- Ensure that the conversation's id references a valid resource.
    ADD CONSTRAINT fk_conversation_resource FOREIGN KEY (id) REFERENCES resources(id) ON DELETE CASCADE,
    -- Ensure that the conversation's owner_id references the underlying resource owner id.
    ADD CONSTRAINT fk_conversation_owner_resource FOREIGN KEY (id, owner_id) REFERENCES resources(id, owner_id);

ALTER TABLE organization
    ALTER COLUMN id DROP DEFAULT,
    ADD CONSTRAINT fk_organization_resource FOREIGN KEY (id) REFERENCES resources(id) ON DELETE CASCADE;

-- Drop now-unused resource_type column from resource_permissions table.
ALTER TABLE resource_permissions
    DROP COLUMN resource_type;

-- Rename legacy role names to match the current Role enum.
UPDATE resource_permissions SET role_name = 'editor' WHERE role_name = 'content_editor';
UPDATE resource_permissions SET role_name = 'admin' WHERE role_name = 'organization_admin';
UPDATE resource_permissions SET role_name = 'viewer' WHERE role_name = 'conversation_co_host';

