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

set -euo pipefail

BACKEND_URL="${API_URL:-http://localhost:3000}"
ADMIN_EMAIL="${ADMIN_EMAIL:-admin@crown-shy.com}"
ADMIN_PASSWORD="${ADMIN_PASSWORD:-adminPassword123!}"
ADMIN_USERNAME="${ADMIN_USERNAME:-devadmin}"

command -v curl >/dev/null || { echo "missing curl"; exit 1; }
command -v jq   >/dev/null || { echo "missing jq";   exit 1; }

login() {
  curl -s -i -X POST "$BACKEND_URL/auth/login" \
    -H "Content-Type: application/json" \
    --data "$(jq -nc --arg e "$ADMIN_EMAIL" --arg p "$ADMIN_PASSWORD" '{email:$e,password:$p}')" \
    | tr -d '\r' \
    | awk -F'auth-token=' 'tolower($0) ~ /^set-cookie:/ && NF>1 { sub(/;.*/, "", $2); print $2; exit }'
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
