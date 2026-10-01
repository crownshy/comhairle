CREATE TABLE
    user_group (
        id UUID PRIMARY KEY DEFAULT gen_random_uuid (),
        name TEXT NOT NULL,
        created_at TIMESTAMPTZ NOT NULL DEFAULT now()
    );

ALTER TABLE organization
ADD COLUMN user_group_id UUID UNIQUE REFERENCES user_group (id);

INSERT INTO
    user_group (id, name)
SELECT
    id,
    name
FROM
    organization;

UPDATE organization
SET
    user_group_id = id;

ALTER TABLE organization
ALTER COLUMN user_group_id
SET NOT NULL;

CREATE FUNCTION create_organization_membership_group () RETURNS trigger LANGUAGE plpgsql AS $$
BEGIN
    IF NEW.user_group_id IS NULL THEN
        INSERT INTO user_group (name) VALUES (NEW.name) RETURNING id INTO NEW.user_group_id;
    END IF;
    RETURN NEW;
END;
$$;

CREATE TRIGGER organization_membership_group BEFORE INSERT ON organization FOR EACH ROW
EXECUTE FUNCTION create_organization_membership_group ();

CREATE TABLE
    user_group_member (
        group_id UUID NOT NULL REFERENCES user_group (id) ON DELETE CASCADE,
        user_id UUID NOT NULL REFERENCES comhairle_user (id) ON DELETE CASCADE,
        PRIMARY KEY (group_id, user_id)
    );

CREATE INDEX user_group_member_user ON user_group_member (user_id, group_id);

INSERT INTO
    user_group_member (group_id, user_id)
SELECT
    organization.user_group_id,
    comhairle_user.id
FROM
    comhairle_user
    JOIN organization ON organization.id = comhairle_user.organization_id;

CREATE TABLE
    permission_set_version (
        resource_type TEXT NOT NULL,
        resource_id UUID NOT NULL,
        recipient_type TEXT NOT NULL CHECK (recipient_type IN ('user', 'group')),
        recipient_id UUID NOT NULL,
        version BIGINT NOT NULL DEFAULT 1 CHECK (version > 0),
        PRIMARY KEY (
            resource_type,
            resource_id,
            recipient_type,
            recipient_id
        )
    );

CREATE TABLE
    user_group_membership_version (
        user_id UUID PRIMARY KEY,
        version BIGINT NOT NULL DEFAULT 1 CHECK (version > 0)
    );

CREATE FUNCTION advance_membership_version () RETURNS trigger LANGUAGE plpgsql AS $$
DECLARE affected_user UUID;
BEGIN
    IF TG_OP = 'DELETE' THEN affected_user := OLD.user_id;
    ELSE affected_user := NEW.user_id;
    END IF;
    INSERT INTO user_group_membership_version (user_id) VALUES (affected_user)
    ON CONFLICT (user_id) DO UPDATE SET version = user_group_membership_version.version + 1;
    IF TG_OP = 'UPDATE' AND OLD.user_id <> NEW.user_id THEN
        INSERT INTO user_group_membership_version (user_id) VALUES (OLD.user_id)
        ON CONFLICT (user_id) DO UPDATE SET version = user_group_membership_version.version + 1;
    END IF;
    RETURN NULL;
END;
$$;

CREATE TRIGGER membership_version
AFTER INSERT
OR
UPDATE
OR DELETE ON user_group_member FOR EACH ROW
EXECUTE FUNCTION advance_membership_version ();

CREATE FUNCTION advance_permission_version () RETURNS trigger LANGUAGE plpgsql AS $$
DECLARE assignment JSONB; affected_recipient UUID; affected_resource UUID;
BEGIN
    FOR assignment IN
        SELECT to_jsonb(OLD) WHERE TG_OP IN ('DELETE', 'UPDATE')
        UNION ALL SELECT to_jsonb(NEW) WHERE TG_OP IN ('INSERT', 'UPDATE')
    LOOP
        affected_recipient := (assignment ->> TG_ARGV[2])::UUID;
        affected_resource := CASE WHEN TG_ARGV[0] = 'system'
            THEN '00000000-0000-0000-0000-000000000000'::UUID
            ELSE (assignment ->> 'resource_id')::UUID END;
        INSERT INTO permission_set_version (resource_type, resource_id, recipient_type, recipient_id)
        VALUES (TG_ARGV[0], affected_resource, TG_ARGV[1], affected_recipient)
        ON CONFLICT (resource_type, resource_id, recipient_type, recipient_id)
        DO UPDATE SET version = permission_set_version.version + 1;
    END LOOP;
    RETURN NULL;
END;
$$;

DO $$
BEGIN
    IF EXISTS (SELECT 1 FROM resource_permissions WHERE resource_type NOT IN ('conversation', 'organization', 'system')) THEN
        RAISE EXCEPTION 'Unsupported legacy permission resource types must be migrated before splitting permission tables';
    END IF;
END;
$$;

DO $$
DECLARE resource_kind TEXT; recipient_kind TEXT; assignment_table TEXT; recipient_column TEXT;
    resource_column TEXT; resource_key TEXT;
BEGIN
    FOREACH resource_kind IN ARRAY ARRAY['conversation', 'organization', 'system'] LOOP
    resource_column := CASE WHEN resource_kind = 'system' THEN '' ELSE 'resource_id UUID NOT NULL,' END;
    resource_key := CASE WHEN resource_kind = 'system' THEN '' ELSE 'resource_id,' END;
        FOREACH recipient_kind IN ARRAY ARRAY['user', 'group'] LOOP
            assignment_table := resource_kind || '_' || recipient_kind || '_permissions';
            recipient_column := CASE recipient_kind WHEN 'user' THEN 'user_id' ELSE 'group_id' END;
            EXECUTE format(
                'CREATE TABLE %I (
                    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
                    %I UUID NOT NULL REFERENCES %I(id) ON DELETE CASCADE,
                    %s
                    role_name VARCHAR(50) NOT NULL,
                    granted_by UUID REFERENCES comhairle_user(id) ON DELETE SET NULL,
                    grant_reason TEXT NOT NULL,
                    granted_at TIMESTAMPTZ NOT NULL DEFAULT now(),
                    UNIQUE (%I, %s role_name)
                )', assignment_table, recipient_column,
                CASE recipient_kind WHEN 'user' THEN 'comhairle_user' ELSE 'user_group' END,
                resource_column, recipient_column, resource_key
            );
            EXECUTE format('CREATE INDEX ON %I (%s granted_at DESC, id DESC)', assignment_table, resource_key);
            EXECUTE format(
                'CREATE TRIGGER permission_version AFTER INSERT OR UPDATE OR DELETE ON %I
                 FOR EACH ROW EXECUTE FUNCTION advance_permission_version(%L, %L, %L)',
                assignment_table, resource_kind, recipient_kind, recipient_column
            );
            IF recipient_kind = 'user' THEN
                EXECUTE format(
                    'INSERT INTO %I SELECT id, user_id, %s role_name, granted_by, grant_reason, granted_at
                     FROM resource_permissions WHERE resource_type = %L AND user_id IS NOT NULL',
                    assignment_table, resource_key, resource_kind
                );
            ELSE
                EXECUTE format(
                    'INSERT INTO %I SELECT permission.id, organization.user_group_id, %s role_name,
                     granted_by, grant_reason, granted_at FROM resource_permissions permission
                     JOIN organization ON organization.id = permission.organization_id
                    WHERE resource_type = %L', assignment_table, resource_key, resource_kind
                );
            END IF;
        END LOOP;
    END LOOP;
END;
$$;

DROP TABLE resource_permissions;

CREATE FUNCTION synchronize_legacy_organization_membership () RETURNS trigger LANGUAGE plpgsql AS $$
BEGIN
    IF NEW.organization_id IS NOT NULL THEN
        INSERT INTO user_group_member (group_id, user_id)
        SELECT user_group_id, NEW.id FROM organization WHERE id = NEW.organization_id
        ON CONFLICT DO NOTHING;
    ELSIF TG_OP = 'UPDATE' AND OLD.organization_id IS NOT NULL THEN
        DELETE FROM user_group_member WHERE user_id = NEW.id AND group_id =
            (SELECT user_group_id FROM organization WHERE id = OLD.organization_id);
    END IF;
    RETURN NEW;
END;
$$;

CREATE TRIGGER legacy_organization_membership
AFTER INSERT
OR
UPDATE OF organization_id ON comhairle_user FOR EACH ROW
EXECUTE FUNCTION synchronize_legacy_organization_membership ();

CREATE TABLE
    permission_mutation_guard (
        singleton BOOLEAN PRIMARY KEY DEFAULT TRUE CHECK (singleton),
        generation BIGINT NOT NULL DEFAULT 0,
        system_initialized BOOLEAN NOT NULL DEFAULT FALSE
    );

INSERT INTO
    permission_mutation_guard (singleton)
VALUES
    (TRUE);

CREATE FUNCTION effective_superadmin_exists () RETURNS BOOLEAN LANGUAGE sql STABLE AS $$
    SELECT EXISTS (
        SELECT permission.user_id FROM system_user_permissions permission
        JOIN comhairle_user ON comhairle_user.id = permission.user_id
        WHERE role_name = 'super_admin'
        UNION ALL
        SELECT membership.user_id FROM system_group_permissions permission
        JOIN user_group_member membership ON membership.group_id = permission.group_id
        JOIN comhairle_user ON comhairle_user.id = membership.user_id
        WHERE role_name = 'super_admin'
    );
$$;

CREATE FUNCTION assert_administrator_continuity () RETURNS void LANGUAGE plpgsql AS $$
BEGIN
    IF EXISTS (
        SELECT organization.id FROM organization WHERE NOT EXISTS (
            SELECT permission.user_id FROM organization_user_permissions permission
            JOIN comhairle_user ON comhairle_user.id = permission.user_id
            WHERE permission.resource_id = organization.id AND permission.role_name = 'organization_admin'
            UNION ALL
            SELECT membership.user_id FROM organization_group_permissions permission
            JOIN user_group_member membership ON membership.group_id = permission.group_id
            JOIN comhairle_user ON comhairle_user.id = membership.user_id
            WHERE permission.resource_id = organization.id AND permission.role_name = 'organization_admin'
        )
    ) THEN
        RAISE EXCEPTION 'Every Organization must retain an administrator'
            USING ERRCODE = '23514', CONSTRAINT = 'organization_admin_continuity';
    END IF;
    IF effective_superadmin_exists() THEN
        UPDATE permission_mutation_guard SET system_initialized = TRUE
        WHERE singleton AND NOT system_initialized;
    ELSIF (SELECT system_initialized FROM permission_mutation_guard WHERE singleton) THEN
        RAISE EXCEPTION 'At least one actual User must retain SuperAdmin'
            USING ERRCODE = '23514', CONSTRAINT = 'platform_superadmin_continuity';
    END IF;
END;
$$;

SELECT
    assert_administrator_continuity ();

CREATE FUNCTION serialize_permission_mutation () RETURNS trigger LANGUAGE plpgsql AS $$
BEGIN
    UPDATE permission_mutation_guard SET generation = generation + 1 WHERE singleton;
    RETURN NULL;
END;
$$;

CREATE FUNCTION validate_permission_mutation () RETURNS trigger LANGUAGE plpgsql AS $$
BEGIN
    PERFORM assert_administrator_continuity();
    RETURN NULL;
END;
$$;

DO $$
DECLARE guarded_table TEXT;
BEGIN
    FOREACH guarded_table IN ARRAY ARRAY[
        'organization', 'comhairle_user', 'user_group', 'user_group_member',
        'organization_user_permissions', 'organization_group_permissions',
        'system_user_permissions', 'system_group_permissions'
    ] LOOP
        EXECUTE format('CREATE TRIGGER serialize_permission_mutation
            BEFORE INSERT OR UPDATE OR DELETE ON %I FOR EACH STATEMENT
            EXECUTE FUNCTION serialize_permission_mutation()', guarded_table);
        EXECUTE format('CREATE CONSTRAINT TRIGGER administrator_continuity
            AFTER INSERT OR UPDATE OR DELETE ON %I DEFERRABLE INITIALLY DEFERRED
            FOR EACH ROW EXECUTE FUNCTION validate_permission_mutation()', guarded_table);
    END LOOP;
END;
$$;