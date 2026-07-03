#!/usr/bin/env bash
# Backfill system admin role into resource_permissions for existing admin users.
#
# Finds users in comhairle_user whose email matches ADMIN_USERS_REGEX and inserts
# a system admin permission row for each one, skipping any that already have it.
#
# Usage: ./scripts/backfill-admin-permissions.sh
#
# Env:
#   DATABASE_URL        Postgres connection string (required)
#                       e.g. postgres://user:pass@localhost:5432/comhairle
#   ADMIN_USERS_REGEX   Postgres regular expression matched against email (required)
#                       e.g. '@example\.com$' or '^(alice|bob)@example\.com$'

set -euo pipefail

: "${DATABASE_URL:?DATABASE_URL must be set}"
: "${ADMIN_USERS_REGEX:?ADMIN_USERS_REGEX must be set (Postgres regex matched against email)}"

command -v psql >/dev/null || { echo "missing psql"; exit 1; }

echo "→ Checking users matching: $ADMIN_USERS_REGEX"

# Dry-run: show which users would be affected and how many lack the role already
AFFECTED=$(psql "$DATABASE_URL" --tuples-only --no-align <<SQL
SELECT COUNT(*), string_agg(email, ', ' ORDER BY email)
FROM comhairle_user
WHERE LOWER(email) ~* ${ADMIN_USERS_REGEX@Q}
  AND id NOT IN (
      SELECT user_id
      FROM resource_permissions
      WHERE resource_type = 'system'
        AND resource_id   = '00000000-0000-0000-0000-000000000000'
        AND role_name     = 'admin'
        AND user_id IS NOT NULL
  );
SQL
)

AFFECTED_COUNT=$(echo "$AFFECTED" | awk -F'|' '{print $1}' | xargs)
AFFECTED_EMAILS=$(echo "$AFFECTED" | awk -F'|' '{print $2}' | xargs)

if [[ "$AFFECTED_COUNT" -eq 0 ]]; then
    echo "No users to backfill — all matching users already have the system admin role."
    exit 0
fi

echo ""
echo "WARNING: This will grant the system admin role to $AFFECTED_COUNT user(s):"
echo "$AFFECTED_EMAILS"
echo ""
read -r -p "Type 'yes' to continue: " CONFIRM

if [[ "$CONFIRM" != "yes" ]]; then
    echo "Aborted."
    exit 1
fi

echo ""
echo "→ Backfilling system admin role for emails matching: $ADMIN_USERS_REGEX"

GRANTED=$(psql "$DATABASE_URL" --tuples-only --no-align <<SQL
WITH target_users AS (
    SELECT id
    FROM comhairle_user
    WHERE LOWER(email) ~* ${ADMIN_USERS_REGEX@Q}
),
grantor_user AS (
    SELECT id
    FROM comhairle_user
    WHERE LOWER(email) = 'admin@crown-shy.com'
    LIMIT 1
),
selected_grantor AS (
    SELECT COALESCE(
        (SELECT id FROM grantor_user),
        (SELECT id FROM target_users ORDER BY id LIMIT 1)
    ) AS id
),
inserted AS (
    INSERT INTO resource_permissions (
        user_id,
        resource_id,
        resource_type,
        role_name,
        granted_by,
        grant_reason,
        granted_at
    )
    SELECT
        id,
        '00000000-0000-0000-0000-000000000000'::UUID,
        'system',
        'admin',
        (SELECT id FROM selected_grantor),
        'Backfilled by backfill-admin-permissions script',
        NOW()
    FROM target_users
    ON CONFLICT DO NOTHING
    RETURNING user_id
)
SELECT COUNT(*) FROM inserted;
SQL
)

GRANTED=$(echo "$GRANTED" | xargs)
echo "Granted $GRANTED new system admin permission(s)"
