#!/bin/bash
set -e

# Test script for the Jitsi Recorder Web Service
# This script tests the API endpoints

BASE_URL="${BASE_URL:-http://localhost:3000}"

echo "Testing Jitsi Recorder Web Service API"
echo "========================================"
echo "Base URL: $BASE_URL"
echo ""

# Check if server is running
echo "1. Checking if server is running..."
if ! curl -s -f "$BASE_URL/recordings" > /dev/null; then
    echo "❌ Error: Server is not responding at $BASE_URL"
    echo "   Please start the server with: cargo run --release"
    exit 1
fi
echo "✓ Server is running"
echo ""

# Start a test recording
echo "2. Starting a test recording..."
echo "   Note: No JWT token provided - using anonymous authentication"
RESPONSE=$(curl -s -X POST "$BASE_URL/recordings" \
    -H "Content-Type: application/json" \
    -d '{
        "room_name": "test-room-'$(date +%s)'",
        "s3_bucket": "test-bucket",
        "s3_path": "test/recording.wav",
        "webhook_url": "https://webhook.site/test",
        "nickname": "test-bot"
    }')

SESSION_ID=$(echo "$RESPONSE" | jq -r '.session_id')
ROOM_NAME=$(echo "$RESPONSE" | jq -r '.room_name')
STATUS=$(echo "$RESPONSE" | jq -r '.status')

if [ "$SESSION_ID" = "null" ] || [ -z "$SESSION_ID" ]; then
    echo "❌ Failed to start recording"
    echo "Response: $RESPONSE"
    exit 1
fi

echo "✓ Recording started"
echo "  Session ID: $SESSION_ID"
echo "  Room: $ROOM_NAME"
echo "  Status: $STATUS"
echo ""

# Get recording details
echo "3. Getting recording details..."
DETAIL_RESPONSE=$(curl -s "$BASE_URL/recordings/$SESSION_ID")
DETAIL_STATUS=$(echo "$DETAIL_RESPONSE" | jq -r '.status')

echo "✓ Retrieved recording details"
echo "  Status: $DETAIL_STATUS"
echo ""

# List all recordings
echo "4. Listing all recordings..."
LIST_RESPONSE=$(curl -s "$BASE_URL/recordings")
RECORDING_COUNT=$(echo "$LIST_RESPONSE" | jq '.recordings | length')

echo "✓ Listed recordings"
echo "  Total recordings: $RECORDING_COUNT"
echo ""

# Display the recording in the list
echo "5. Finding our recording in the list..."
FOUND=$(echo "$LIST_RESPONSE" | jq --arg sid "$SESSION_ID" '.recordings[] | select(.session_id == $sid)')

if [ -z "$FOUND" ]; then
    echo "❌ Recording not found in list"
    exit 1
fi

echo "✓ Recording found in list"
echo ""

echo "========================================"
echo "All tests passed! ✓"
echo ""
echo "Session ID: $SESSION_ID"
echo "Room Name: $ROOM_NAME"
echo ""
echo "Next steps:"
echo "1. Join the Jitsi room: https://jitsi.comhairle.scot/$ROOM_NAME"
echo "2. Speak into your microphone"
echo "3. Leave the room to end the recording"
echo "4. Check the recording status:"
echo "   curl $BASE_URL/recordings/$SESSION_ID | jq"
echo ""
echo "Note: The gst-meet process may fail without proper Jitsi credentials."
echo "      Check the server logs for details."
