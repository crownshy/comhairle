#!/usr/bin/env bash
# Quick test of gst-meet for recording Jitsi audio

ROOM_NAME="${1:-f52815c9-102a-4cfd-a914-35b69fb8b009}"
OUTPUT_FILE="test_recording_$(date +%Y%m%d_%H%M%S).wav"

echo "Testing gst-meet audio recording"
echo "Room: $ROOM_NAME"
echo "Output: $OUTPUT_FILE"
echo ""

# Load JWT settings if available
if [ -f .env ]; then
    source .env
elif [ -f /home/stuart/crown_shy/comhairle/jisti-recorder/.env ]; then
    source /home/stuart/crown_shy/comhairle/jisti-recorder/.env
fi

# Generate JWT token if configured
WEBSOCKET_URL="wss://jitsi.comhairle.scot/xmpp-websocket"
# if [ "$JITSI_USE_JWT" = "true" ] && [ -n "$JITSI_APP_ID" ] && [ -n "$JITSI_APP_SECRET" ]; then
#     echo "Generating JWT token..."
#     # Use the Rust binary to generate the token and extract it
#     JWT_TOKEN=$(cd /home/stuart/crown_shy/comhairle/jisti-recorder && cargo run --quiet -- --help 2>&1 | grep -o 'eyJ[^"]*' | head -1)
#
#     # Actually, let's just use anonymous auth for testing
#     # JWT generation is complex - the Rust code can do it
#     echo "Note: JWT auth configured but using anonymous for this test"
#     echo "If anonymous doesn't work, you'll need to use the Rust binary with JWT"
# fi
#
# Build command
# CRITICAL: The XMPP domain must be "meet.jitsi" not "jitsi.comhairle.scot"
# Based on working config from the Rust version
CMD=(
    ~/.cargo/bin/gst-meet
    --web-socket-url="$WEBSOCKET_URL"
    --xmpp-domain="meet.jitsi"
    --muc-domain="muc.meet.jitsi"
    --focus-jid="focus.meet.jitsi"
    --room-name="$ROOM_NAME"
    --nick="gst-meet-recorder"
    --xmpp-jwt="eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJzdWIiOiJqaXRzaS5jb21oYWlybGUuc2NvdCIsImV4cCI6MTc3NjE5ODE5MywiaWQiOiJmNjRjZDdmMy01MWIyLTRmZDctOGRjNS1iODc1ZTQ3ZmIyNTMiLCJpc3MiOiJjb21oYWlybGUuc2NvdCIsImF1ZCI6ImNvbWhhaXJsZS5zY290Iiwicm9vbSI6ImY1MjgxNWM5LTEwMmEtNGNmZC1hOTE0LTM1YjY5ZmI4YjAwOSIsImNvbnRleHQiOnsidXNlciI6eyJuYW1lIjoiYWRtaW4iLCJpZCI6ImY2NGNkN2YzLTUxYjItNGZkNy04ZGM1LWI4NzVlNDdmYjI1MyIsIm1vZGVyYXRvciI6ZmFsc2V9fX0.Jo9aJtNyaRJNNS1HCBR8nMPhEcla89_SVXM90Ad3jSE"
    --verbose
)

# Add recording pipeline - mixed audio from all participants
CMD+=(
    --recv-pipeline="audiomixer name=audio ! audioconvert ! audioresample ! wavenc ! filesink location=$OUTPUT_FILE"
)

echo "Running: ${CMD[@]}"
echo ""
echo "Press Ctrl+C to stop recording"
echo ""

"${CMD[@]}"

echo ""
echo "Recording saved to: $OUTPUT_FILE"
ls -lh "$OUTPUT_FILE"
