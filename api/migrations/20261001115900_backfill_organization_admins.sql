-- Every Organization must retain an administrator (enforced by 20261001120000). Give any
-- Organization that predates the rule one: admin@crown-shy.com, or else the earliest
-- @crown-shy.com user holding a system role.
--
-- Databases where the permission split has already happened (resource_permissions is gone)
-- use the split tables instead. PL/pgSQL only plans the branch it executes.
DO $$
DECLARE
    backfill_admin UUID;
    has_legacy_table BOOLEAN := to_regclass('resource_permissions') IS NOT NULL;
    has_split_tables BOOLEAN := to_regclass('organization_user_permissions') IS NOT NULL
        AND to_regclass('system_user_permissions') IS NOT NULL;
BEGIN
    IF NOT has_legacy_table AND NOT has_split_tables THEN
        RAISE NOTICE 'No permission tables found; nothing to backfill';
        RETURN;
    END IF;

    SELECT id INTO backfill_admin FROM comhairle_user WHERE LOWER(email) = 'admin@crown-shy.com';

    IF backfill_admin IS NULL AND has_legacy_table THEN
        SELECT comhairle_user.id INTO backfill_admin
        FROM comhairle_user
        WHERE LOWER(comhairle_user.email) LIKE '%@crown-shy.com'
          AND EXISTS (
              SELECT 1 FROM resource_permissions permission
              WHERE permission.user_id = comhairle_user.id
                AND permission.resource_type = 'system'
                AND permission.role_name IN ('admin', 'super_admin')
          )
        ORDER BY comhairle_user.created_at, comhairle_user.id
        LIMIT 1;
    ELSIF backfill_admin IS NULL THEN
        SELECT comhairle_user.id INTO backfill_admin
        FROM comhairle_user
        WHERE LOWER(comhairle_user.email) LIKE '%@crown-shy.com'
          AND EXISTS (
              SELECT 1 FROM system_user_permissions permission
              WHERE permission.user_id = comhairle_user.id
                AND permission.role_name IN ('admin', 'super_admin')
          )
        ORDER BY comhairle_user.created_at, comhairle_user.id
        LIMIT 1;
    END IF;

    IF backfill_admin IS NULL THEN
        RAISE NOTICE 'No crown-shy.com admin user found; Organizations without an administrator were not backfilled';
        RETURN;
    END IF;

    IF has_legacy_table THEN
        INSERT INTO resource_permissions (user_id, resource_id, resource_type, role_name, granted_by, grant_reason)
        SELECT backfill_admin, organization.id, 'organization', 'organization_admin', backfill_admin,
               'Backfilled so every Organization has an administrator'
        FROM organization
        WHERE NOT EXISTS (
            SELECT 1 FROM resource_permissions permission
            JOIN comhairle_user ON comhairle_user.id = permission.user_id
            WHERE permission.resource_type = 'organization'
              AND permission.resource_id = organization.id
              AND permission.role_name = 'organization_admin'
        )
        AND NOT EXISTS (
            SELECT 1 FROM resource_permissions permission
            JOIN comhairle_user ON comhairle_user.organization_id = permission.organization_id
            WHERE permission.resource_type = 'organization'
              AND permission.resource_id = organization.id
              AND permission.role_name = 'organization_admin'
        )
        ON CONFLICT DO NOTHING;
    ELSE
        INSERT INTO organization_user_permissions (user_id, resource_id, role_name, granted_by, grant_reason)
        SELECT backfill_admin, organization.id, 'organization_admin', backfill_admin,
               'Backfilled so every Organization has an administrator'
        FROM organization
        WHERE NOT EXISTS (
            SELECT 1 FROM organization_user_permissions permission
            JOIN comhairle_user ON comhairle_user.id = permission.user_id
            WHERE permission.resource_id = organization.id
              AND permission.role_name = 'organization_admin'
        )
        AND NOT EXISTS (
            SELECT 1 FROM organization_group_permissions permission
            JOIN user_group_member membership ON membership.group_id = permission.group_id
            JOIN comhairle_user ON comhairle_user.id = membership.user_id
            WHERE permission.resource_id = organization.id
              AND permission.role_name = 'organization_admin'
        )
        ON CONFLICT DO NOTHING;
    END IF;
END;
$$;
