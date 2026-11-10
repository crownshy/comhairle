#!/bin/bash
# Jitsi Audio Recorder using gst-meet CLI
# Usage: ./record-jitsi.sh <room_name>

ROOM_NAME="${1:-test_room}"
TIMESTAMP=$(date +%Y%m%d_%H%M%S)
OUTPUT_DIR="./recordings/${ROOM_NAME}_${TIMESTAMP}"
mkdir -p "$OUTPUT_DIR"

# Load JWT config if .env exists
if [ -f .env ]; then
    export $(grep -v '^#' .env | xargs)
fi

# Build WebSocket URL with JWT if configured
if [ "$JITSI_USE_JWT" = "true" ]; then
    echo "JWT authentication enabled"
    # Note: gst-meet doesn't support JWT directly in the URL
    # You'll need to modify this based on your Jitsi setup
    WEBSOCKET_URL="wss://jitsi.comhairle.scot/xmpp-websocket"
else
    WEBSOCKET_URL="wss://jitsi.comhairle.scot/xmpp-websocket"
fi

echo "Recording room: $ROOM_NAME"
echo "Output directory: $OUTPUT_DIR"
echo ""
echo "Choose recording mode:"
echo "1) Mixed audio (all participants in one file)"
echo "2) Separate files per participant"
read -p "Enter choice [1-2]: " choice

case $choice in
    1)
        echo "Recording mixed audio to ${OUTPUT_DIR}/mixed.wav"

        # Mixed audio: all participants combined into one WAV file
        ~/.cargo/bin/gst-meet \
            --web-socket-url="$WEBSOCKET_URL" \
            --room-name="$ROOM_NAME" \
            --nickname="recorder-bot" \
            --recv-pipeline="audiomixer name=audio ! \
                            audioconvert ! \
                            audioresample ! \
                            wavenc ! \
                            filesink location=${OUTPUT_DIR}/mixed.wav"
        ;;
    2)
        echo "Recording separate files per participant to ${OUTPUT_DIR}/"

        # Per-participant recording: separate WAV file for each participant
        ~/.cargo/bin/gst-meet \
            --web-socket-url="$WEBSOCKET_URL" \
            --room-name="$ROOM_NAME" \
            --nickname="recorder-bot" \
            --recv-pipeline-participant-template="audioconvert name=audio ! \
                                                   audioresample ! \
                                                   wavenc ! \
                                                   filesink location=${OUTPUT_DIR}/{participant_id}.wav"
        ;;
    *)
        echo "Invalid choice"
        exit 1
        ;;
esac

echo ""
echo "Recording stopped. Files saved to: $OUTPUT_DIR"
