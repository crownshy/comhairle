#!/usr/bin/env bash
# Seed a mock "Green Spaces" conversation (Learn + Polis + Survey steps) via the API.
# Safe to re-run: each run creates a NEW conversation (slug gets a random suffix).
#
# Usage:  ./scripts/seed-green-spaces.sh
# Needs:  API running (just api-dev) and the admin seeded (just seed)
#
# Env:
#   API_URL         (default http://localhost:3000)
#   ADMIN_EMAIL     (default admin@crown-shy.com)
#   ADMIN_PASSWORD  (default adminPassword123!)
#   LAUNCH=1        also make the conversation live (default: stays draft / preview)

set -euo pipefail

API="${API_URL:-http://localhost:3000}"
ADMIN_EMAIL="${ADMIN_EMAIL:-admin@crown-shy.com}"
ADMIN_PASSWORD="${ADMIN_PASSWORD:-adminPassword123!}"

command -v curl >/dev/null || { echo "missing curl"; exit 1; }
command -v jq   >/dev/null || { echo "missing jq";   exit 1; }

# --- auth -------------------------------------------------------------------
TOKEN=$(curl -s -i -X POST "$API/auth/login" -H "Content-Type: application/json" \
  --data "$(jq -nc --arg e "$ADMIN_EMAIL" --arg p "$ADMIN_PASSWORD" '{email:$e,password:$p}')" \
  | tr -d '\r' \
  | awk -F'auth-token=' 'tolower($0) ~ /^set-cookie:/ && NF>1 { sub(/;.*/, "", $2); print $2; exit }')
[ -n "$TOKEN" ] || { echo "❌ login failed — run 'just seed' first"; exit 1; }
echo "✅ logged in as $ADMIN_EMAIL"

# call METHOD PATH [JSON] -> prints body, fails loudly on non-2xx
call() {
  local method=$1 path=$2 body=${3:-}
  local out code
  out=$(curl -s -w $'\n%{http_code}' -X "$method" "$API$path" \
    -H "Content-Type: application/json" -b "auth-token=$TOKEN" \
    ${body:+--data "$body"})
  code=${out##*$'\n'}; out=${out%$'\n'*}
  if [[ "$code" != 2* ]]; then
    echo "❌ $method $path -> $code: $out" >&2; return 1
  fi
  printf '%s' "$out"
}

# --- conversation -----------------------------------------------------------
SLUG="green-spaces-$RANDOM"
CONV=$(call POST /conversation "$(jq -nc --arg slug "$SLUG" '{
  title: "Our Parks and Green Spaces",
  short_description: "Help decide how Ashford Vale cares for, protects and improves its parks, verges and woodland.",
  description: "Ashford Vale has 14 parks, two community woods and miles of grass verges. Budgets are tight, and the council has to choose between longer opening hours, more play equipment, wildflower planting, tree cover and new community gardens.\n\nBefore the Parks Strategy 2027–2032 is written, we want to hear how you use green space today, what you would protect, and what you would change. Your input will be summarised in a public report.",
  slug: $slug,
  primary_locale: "en",
  supported_languages: ["en", "gd"],
  is_public: true,
  is_invite_only: false,
  is_live: false,
  tags: ["environment", "parks", "local-council"]
}')")
CONV_ID=$(jq -r .id <<<"$CONV")
echo "✅ conversation $CONV_ID ($SLUG)"

# --- workflow ---------------------------------------------------------------
WF=$(call POST "/conversation/$CONV_ID/workflow" '{
  "name": "Main workflow", "description": "Green spaces consultation",
  "is_active": true, "is_public": true, "auto_login": true
}')
WF_ID=$(jq -r .id <<<"$WF")
echo "✅ workflow $WF_ID"

step() { # step ORDER NAME DESCRIPTION TOOL_SETUP_JSON REQUIRED
  call POST "/conversation/$CONV_ID/workflow/$WF_ID/workflow_step" "$(jq -nc \
    --arg n "$2" --arg d "$3" --argjson o "$1" --argjson t "$4" --argjson r "$5" \
    '{name:$n, description:$d, step_order:$o, tool_setup:$t, required:$r, is_offline:false, activation_rule:"manual"}')"
}

# --- step 1: learn ----------------------------------------------------------
LEARN_SETUP=$(jq -nc '{type:"learn", pages: [
 [ {type:"markdown", lang:"en", content:"# Why green space matters\n\nParks, woods and verges do far more than look nice. They:\n\n- **Cool the town** – tree-lined streets can be several degrees cooler on hot days.\n- **Soak up rain** – grass and soil absorb water that would otherwise flood drains.\n- **Support wildlife** – hedgerows and wildflower verges are corridors for bees, birds and hedgehogs.\n- **Help us feel better** – regular time outdoors is linked to lower stress and better sleep."},
   {type:"markdown", lang:"gd", content:"# Carson a tha àite uaine cudromach\n\nTha pàircean, coilltean agus oirean-rathaid a’ fuarachadh a’ bhaile, a’ gabhail a-steach uisge, a’ toirt taic do dh’fhiadh-bheathaichean agus gar cuideachadh a bhith a’ faireachdainn nas fheàrr."} ],
 [ {type:"markdown", lang:"en", content:"# Ashford Vale in numbers\n\n*Illustrative figures for testing.*\n\n| | |\n|---|---|\n| Parks | 14 |\n| Community woods | 2 |\n| Tree canopy cover | 17% (target 25%) |\n| Homes more than 10 min walk from a park | 1 in 5 |\n| Annual parks budget | £1.2m (down 18% since 2020) |\n\nNorthfield and the Station Quarter have the least green space per person."},
   {type:"markdown", lang:"gd", content:"# Ashford Vale ann an àireamhan\n\n14 pàircean, 2 choille coimhearsnachd, 17% còmhdach chraobhan (targaid 25%)."} ],
 [ {type:"markdown", lang:"en", content:"# The choices ahead\n\nThe council is weighing up five options:\n\n1. **Plant 5,000 street trees** over five years.\n2. **Let verges grow wild** – fewer mows, more wildflowers.\n3. **New community gardens** on unused council land.\n4. **Better play and fitness equipment** in every park.\n5. **Longer opening hours and more lighting** for winter evenings.\n\nThere is not enough money for everything. Next, tell us what you think."},
   {type:"markdown", lang:"gd", content:"# Na roghainnean\n\nCraobhan sràide, oirean fiadhaich, gàrraidhean coimhearsnachd, uidheamachd cluiche, agus uairean fosglaidh nas fhaide."} ]
]}')
S1=$(step 1 "Learn about our green spaces" "A short introduction to the parks, woods and verges of Ashford Vale." "$LEARN_SETUP" true)
echo "✅ learn step $(jq -r .id <<<"$S1")"

# --- step 2: polis ----------------------------------------------------------
POLIS_SETUP='{"type":"polis","topic":"How should Ashford Vale look after its parks and green spaces?","show_remaining_statements":true,"required_votes":8}'
if S2=$(step 2 "Share and vote on ideas" "Vote on other people’s ideas and add your own." "$POLIS_SETUP" true); then
  S2_ID=$(jq -r .id <<<"$S2")
  echo "✅ polis step $S2_ID"
  SEEDS=(
    "Grass verges should be left to grow wild for pollinators."
    "Long grass on verges looks untidy and neglected."
    "Every street should have at least one tree."
    "Parks should stay open and lit until 10pm in winter."
    "Dogs should be kept on leads in all play areas."
    "Unused council land should be turned into community gardens."
    "I would volunteer a few hours a month to help look after my local park."
    "Money for parks should go first to the areas with the least green space."
    "New housing developments must include public green space."
    "Parks need more benches and public toilets before anything else."
    "Teenagers have nowhere to go in our parks."
    "Charging for car parking at the country park is fair if the money stays in parks."
  )
  n=0
  for s in "${SEEDS[@]}"; do
    if call POST /tools/polis/seed "$(jq -nc --arg w "$S2_ID" --arg t "$s" '{workflow_step_id:$w, statement_text:$t}')" >/dev/null; then n=$((n+1)); fi
  done
  echo "✅ posted $n/${#SEEDS[@]} polis seed statements"
  call POST /tools/polis/statement_aux/sync "$(jq -nc --arg w "$S2_ID" '{workflow_step_id:$w}')" >/dev/null \
    && echo "✅ synced polis statements" || echo "⚠️  sync failed — open the step's moderation tab to sync manually"
else
  echo "⚠️  polis step failed (is the Polis service configured in .env?) — continuing"
fi

# --- step 3: survey ---------------------------------------------------------
if S3=$(step 3 "Tell us how you use green space" "A short survey, about 5 minutes." '{"type":"heyform"}' true); then
  echo "✅ survey step $(jq -r .id <<<"$S3")"
  echo "   ↳ add questions in the admin survey editor (see questions list in chat)"
else
  echo "⚠️  survey step failed (is HeyForm configured in .env?) — continuing"
fi

# --- optional launch --------------------------------------------------------
if [[ "${LAUNCH:-0}" == "1" ]]; then
  call PUT "/conversation/$CONV_ID/launch" >/dev/null && echo "✅ launched"
fi

echo
echo "Admin:   http://localhost:5173/admin/conversations/$CONV_ID/design"
echo "Preview: http://localhost:5173/conversations/$SLUG/preview"
