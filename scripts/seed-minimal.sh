#!/usr/bin/env bash
# Add admin user 
#
# Usage: ./scripts/seed-minimal.sh
#
# Env:
#   API_URL         (default http://localhost:3000)
#   ADMIN_EMAIL     (default admin@crown-shy.com)
#   ADMIN_PASSWORD  (default adminPassword123!)
#   ADMIN_USERNAME  (default devadmin)
#   DATABASE_URL    (default postgres://comhairle:comhairle@localhost:5434/comhairle)

set -euo pipefail

BACKEND_URL="${API_URL:-http://localhost:3000}"
ADMIN_EMAIL="${ADMIN_EMAIL:-admin@crown-shy.com}"
ADMIN_PASSWORD="${ADMIN_PASSWORD:-adminPassword123!}"
ADMIN_USERNAME="${ADMIN_USERNAME:-devadmin}"
DATABASE_URL="${DATABASE_URL:-postgres://comhairle:comhairle@localhost:5434/comhairle}"

command -v curl >/dev/null || { echo "missing curl"; exit 1; }
command -v jq   >/dev/null || { echo "missing jq";   exit 1; }
command -v psql >/dev/null || { echo "missing psql"; exit 1; }

login() {
  curl -s -i -X POST "$BACKEND_URL/auth/login" \
    -H "Content-Type: application/json" \
    --data "$(jq -nc --arg e "$ADMIN_EMAIL" --arg p "$ADMIN_PASSWORD" '{email:$e,password:$p}')" \
    | tr -d '\r' \
    | awk -F'auth-token=' 'tolower($0) ~ /^set-cookie:/ && NF>1 { sub(/;.*/, "", $2); print $2; exit }'
}

grant_super_admin() {
  local granted
  granted=$(psql "$DATABASE_URL" --tuples-only --no-align <<SQL
WITH target_user AS (
    SELECT id
    FROM comhairle_user
    WHERE LOWER(email) = LOWER(${ADMIN_EMAIL@Q})
    LIMIT 1
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
        'super_admin',
        id,
        'Seeded by seed-minimal script',
        NOW()
    FROM target_user
    ON CONFLICT DO NOTHING
    RETURNING user_id
)
SELECT COUNT(*) FROM inserted;
SQL
)

  granted=$(echo "$granted" | xargs)
  if [[ "$granted" == "1" ]]; then
    echo "✅ granted super_admin to $ADMIN_EMAIL"
  else
    echo "✅ super_admin already present for $ADMIN_EMAIL"
  fi
}

echo "→ login as $ADMIN_EMAIL"
AUTH_COOKIE=$(login || true)

if [ -z "$AUTH_COOKIE" ]; then
  echo "→ no session, signing up"
  curl -s -X POST "$BACKEND_URL/auth/signup" \
    -H "Content-Type: application/json" \
    --data "$(jq -nc \
      --arg u "${ADMIN_USERNAME}-$RANDOM" \
      --arg e "$ADMIN_EMAIL" \
      --arg p "$ADMIN_PASSWORD" \
      '{username:$u,email:$e,password:$p}')" >/dev/null
  AUTH_COOKIE=$(login || true)
fi

[ -n "$AUTH_COOKIE" ] || { echo "login failed"; exit 1; }
echo "✅ admin ready: $ADMIN_EMAIL"

echo "→ ensuring super_admin role for $ADMIN_EMAIL"
grant_super_admin
