-- Each Organization gets two User groups: members (organization.user_group_id) and
-- administrators (organization.admin_group_id). The members group holds the view role and
-- the administrators group holds the admin role.
--
-- Organization is altered before any of its rows are touched: PostgreSQL refuses to ALTER a
-- table with pending deferred trigger events, and the continuity triggers defer to commit.
ALTER TABLE organization
ADD COLUMN admin_group_id UUID NOT NULL DEFAULT gen_random_uuid ();

ALTER TABLE organization
ALTER COLUMN admin_group_id
DROP DEFAULT;

INSERT INTO
    user_group (id, name)
SELECT
    admin_group_id,
    name || ' administrators'
FROM
    organization;

ALTER TABLE organization
ADD CONSTRAINT organization_admin_group_id_key UNIQUE (admin_group_id),
ADD CONSTRAINT organization_admin_group_id_fkey FOREIGN KEY (admin_group_id) REFERENCES user_group (id);

-- Direct administrators, and members of a membership group that held the admin role,
-- become members of the administrators group.
INSERT INTO
    user_group_member (group_id, user_id)
SELECT
    organization.admin_group_id,
    permission.user_id
FROM
    organization_user_permissions permission
    JOIN organization ON organization.id = permission.resource_id
WHERE
    permission.role_name = 'organization_admin'
UNION
SELECT
    organization.admin_group_id,
    membership.user_id
FROM
    organization_group_permissions permission
    JOIN organization ON organization.id = permission.resource_id
    AND organization.user_group_id = permission.group_id
    JOIN user_group_member membership ON membership.group_id = permission.group_id
WHERE
    permission.role_name = 'organization_admin';

-- Administrators are always members, so they keep the view role and co-host access.
INSERT INTO
    user_group_member (group_id, user_id)
SELECT
    organization.user_group_id,
    administrator.user_id
FROM
    user_group_member administrator
    JOIN organization ON organization.admin_group_id = administrator.group_id ON CONFLICT DO NOTHING;

DELETE FROM organization_user_permissions
WHERE
    role_name = 'organization_admin';

DELETE FROM organization_group_permissions permission USING organization
WHERE
    permission.resource_id = organization.id
    AND permission.group_id = organization.user_group_id
    AND permission.role_name = 'organization_admin';

INSERT INTO
    organization_group_permissions (group_id, resource_id, role_name, grant_reason)
SELECT
    admin_group_id,
    id,
    'organization_admin',
    'Organization administrators group'
FROM
    organization
UNION ALL
SELECT
    user_group_id,
    id,
    'organization_member',
    'Organization members group'
FROM
    organization;

CREATE OR REPLACE FUNCTION create_organization_membership_group () RETURNS trigger LANGUAGE plpgsql AS $$
BEGIN
    IF NEW.user_group_id IS NULL THEN
        INSERT INTO user_group (name) VALUES (NEW.name) RETURNING id INTO NEW.user_group_id;
    END IF;
    IF NEW.admin_group_id IS NULL THEN
        INSERT INTO user_group (name) VALUES (NEW.name || ' administrators') RETURNING id INTO NEW.admin_group_id;
    END IF;
    RETURN NEW;
END;
$$;

CREATE FUNCTION grant_organization_group_roles () RETURNS trigger LANGUAGE plpgsql AS $$
BEGIN
    INSERT INTO organization_group_permissions (group_id, resource_id, role_name, grant_reason)
    VALUES
        (NEW.admin_group_id, NEW.id, 'organization_admin', 'Organization creation'),
        (NEW.user_group_id, NEW.id, 'organization_member', 'Organization creation');
    RETURN NULL;
END;
$$;

CREATE TRIGGER organization_group_roles
AFTER INSERT ON organization FOR EACH ROW
EXECUTE FUNCTION grant_organization_group_roles ();
