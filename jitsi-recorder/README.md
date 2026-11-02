# Jitsi Recorder Web Service

A web service that manages Jitsi meeting recordings through the gst-meet CLI tool. Provides a REST API for starting, monitoring, and managing recording sessions with automatic S3 upload and webhook notifications.

## Features

- REST API for managing recording sessions
- Spawns gst-meet subprocesses for actual recording
- Redis for session persistence
- Automatic S3 upload when recordings complete
- Webhook notifications on completion
- JWT authentication support for Jitsi

## Architecture

- **Axum HTTP Server** - REST API for managing recordings
- **Redis** - Persistent storage for recording session state
- **gst-meet CLI** - Subprocess for actual recording
- **S3 Client** - Upload completed recordings
- **Webhook Client** - Notify external systems when recordings complete

## Requirements

- Rust 1.70+
- Redis server
- AWS S3 credentials
- gst-meet binary (from the lib-gst-meet project)
- GStreamer 1.0+ with plugins

## Deployment Options

### Option 1: Docker (Recommended)

See [DOCKER.md](DOCKER.md) for complete Docker deployment guide.

**Quick Start:**
```bash
# 1. Copy environment template
cp .env.docker .env

# 2. Add your AWS credentials to .env
nano .env

# 3. Start services
docker-compose up -d

# 4. Test
curl http://localhost:3000/recordings
```

### Option 2: Build from Source

```bash
cargo build --release
```

## Configuration

Copy the `.env` file and update with your values:

```bash
# Server Configuration
PORT=3000

# Jitsi Configuration
JITSI_WEBSOCKET_URL=wss://jitsi.comhairle.scot/xmpp-websocket

# AWS S3 Configuration
AWS_ACCESS_KEY_ID=your_access_key
AWS_SECRET_ACCESS_KEY=your_secret_key
AWS_REGION=us-east-1

# Redis Configuration
REDIS_URL=redis://localhost:6379

# Recording Configuration
GST_MEET_PATH=/path/to/gst-meet
RECORDINGS_DIR=/tmp/jitsi-recordings
```

### Environment Variables

| Variable | Required | Default | Description |
|----------|----------|---------|-------------|
| `PORT` | No | `3000` | HTTP server port |
| `JITSI_WEBSOCKET_URL` | No | `wss://jitsi.comhairle.scot/xmpp-websocket` | Jitsi WebSocket URL |
| `AWS_ACCESS_KEY_ID` | Yes | - | AWS access key |
| `AWS_SECRET_ACCESS_KEY` | Yes | - | AWS secret key |
| `AWS_REGION` | Yes | - | AWS region |
| `REDIS_URL` | No | `redis://localhost:6379` | Redis connection URL |
| `GST_MEET_PATH` | No | `gst-meet` | Path to gst-meet binary |
| `RECORDINGS_DIR` | No | `/tmp/jitsi-recordings` | Local directory for recordings |

## Running

### Start Redis

```bash
# Using Docker
docker run -d -p 6379:6379 redis:latest

# Or using local Redis
redis-server
```

### Start the Server

```bash
./target/release/jisti-recorder
```

The server will start on the configured port (default: 3000).

## JWT Token Generation

The service expects JWT tokens to be generated externally and passed in the API request. This separates authentication concerns and allows you to use your preferred JWT generation method.

### Example JWT Token Generation (Node.js)

```javascript
const jwt = require('jsonwebtoken');

function generateJitsiToken(appId, appSecret, roomName, nickname) {
  const now = Math.floor(Date.now() / 1000);

  const payload = {
    aud: 'comhairle.scot',
    iss: appId,
    sub: 'jitsi.comhairle.scot',
    room: roomName,
    nbf: now,
    exp: now + 3600, // 1 hour
    context: {
      user: {
        name: nickname,
        moderator: false
      }
    }
  };

  return jwt.sign(payload, appSecret, { algorithm: 'HS256' });
}

// Usage
const token = generateJitsiToken(
  'comhairle.scot',
  'your_secret_here',
  'meeting-room-123',
  'recorder-bot'
);
```

### Example JWT Token Generation (Python)

```python
import jwt
import time

def generate_jitsi_token(app_id, app_secret, room_name, nickname):
    now = int(time.time())

    payload = {
        'aud': 'comhairle.scot',
        'iss': app_id,
        'sub': 'jitsi.comhairle.scot',
        'room': room_name,
        'nbf': now,
        'exp': now + 3600,  # 1 hour
        'context': {
            'user': {
                'name': nickname,
                'moderator': False
            }
        }
    }

    return jwt.encode(payload, app_secret, algorithm='HS256')

# Usage
token = generate_jitsi_token(
    'comhairle.scot',
    'your_secret_here',
    'meeting-room-123',
    'recorder-bot'
)
```

## API Documentation

### Start a Recording

**POST /recordings**

Start a new recording session.

```bash
curl -X POST http://localhost:3000/recordings \
  -H "Content-Type: application/json" \
  -d '{
    "room_name": "meeting-room-123",
    "s3_bucket": "my-recordings",
    "s3_path": "recordings/2026/meeting-123.wav",
    "webhook_url": "https://api.example.com/recording-complete",
    "nickname": "recorder-bot",
    "jwt_token": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9..."
  }'
```

**Request Body:**
- `room_name` (string, required) - Jitsi room name to join
- `s3_bucket` (string, required) - S3 bucket for upload
- `s3_path` (string, required) - S3 key/path for the recording
- `webhook_url` (string, required) - URL to notify when recording completes
- `nickname` (string, optional) - Bot nickname (default: "recorder-bot")
- `jwt_token` (string, optional) - JWT token for Jitsi authentication (if not provided, uses anonymous auth)

**Response:** (201 Created)
```json
{
  "session_id": "550e8400-e29b-41d4-a716-446655440000",
  "room_name": "meeting-room-123",
  "status": "recording",
  "started_at": "2026-04-30T10:30:00Z"
}
```

### List All Recordings

**GET /recordings**

List all recording sessions.

```bash
curl http://localhost:3000/recordings
```

**Response:**
```json
{
  "recordings": [
    {
      "session_id": "550e8400-e29b-41d4-a716-446655440000",
      "room_name": "meeting-room-123",
      "status": "completed",
      "started_at": "2026-04-30T10:30:00Z",
      "ended_at": "2026-04-30T11:15:00Z",
      "s3_url": "s3://my-recordings/recordings/2026/meeting-123.wav",
      "duration_seconds": 2700
    }
  ]
}
```

### Get Recording Details

**GET /recordings/{session_id}**

Get details of a specific recording session.

```bash
curl http://localhost:3000/recordings/550e8400-e29b-41d4-a716-446655440000
```

**Response:**
```json
{
  "session_id": "550e8400-e29b-41d4-a716-446655440000",
  "room_name": "meeting-room-123",
  "status": "completed",
  "started_at": "2026-04-30T10:30:00Z",
  "ended_at": "2026-04-30T11:15:00Z",
  "s3_url": "s3://my-recordings/recordings/2026/meeting-123.wav",
  "duration_seconds": 2700
}
```

### Recording Status Values

- `recording` - Currently recording
- `uploading` - Recording finished, uploading to S3
- `completed` - Upload complete, webhook sent
- `failed` - Error occurred (check `error_message` field)

### Webhook Payload

When a recording completes, the service sends a POST request to the configured webhook URL:

```json
{
  "session_id": "550e8400-e29b-41d4-a716-446655440000",
  "room_name": "meeting-room-123",
  "status": "completed",
  "started_at": "2026-04-30T10:30:00Z",
  "ended_at": "2026-04-30T11:15:00Z",
  "s3_url": "s3://my-recordings/recordings/2026/meeting-123.wav",
  "duration_seconds": 2700
}
```

## How It Works

1. Client sends POST request to `/recordings` with room details
2. Server spawns a gst-meet subprocess configured for the room
3. Session metadata is stored in Redis
4. A background task monitors the subprocess
5. When the process exits:
   - File is uploaded to S3 (with 3 retry attempts)
   - Webhook notification is sent
   - Session status is updated to "completed"
   - Local file is cleaned up

## Error Handling

- If gst-meet fails to start, the API returns a 500 error
- If the process crashes, the session is marked as "failed"
- S3 uploads are retried up to 3 times with exponential backoff
- Webhook failures are logged but don't fail the session (webhook is non-critical)
- Failed uploads keep the local file for debugging

## Testing

### Basic Test

```bash
# Start a recording
SESSION_ID=$(curl -s -X POST http://localhost:3000/recordings \
  -H "Content-Type: application/json" \
  -d '{
    "room_name": "test-room",
    "s3_bucket": "my-bucket",
    "s3_path": "test.wav",
    "webhook_url": "https://webhook.site/your-unique-url"
  }' | jq -r '.session_id')

# Check status
curl http://localhost:3000/recordings/$SESSION_ID

# List all recordings
curl http://localhost:3000/recordings
```

### End-to-End Test

1. Start the server
2. Start a recording via the API
3. Join the Jitsi room and speak
4. Leave the room (gst-meet will exit)
5. Verify the file was uploaded to S3
6. Verify the webhook was called
7. Check the session status is "completed"

## Troubleshooting

### Redis Connection Failed

Ensure Redis is running:
```bash
redis-cli ping
# Should return: PONG
```

### S3 Upload Failed

- Verify AWS credentials are set correctly
- Check S3 bucket exists and you have write permissions
- Review CloudWatch logs for detailed error messages

### gst-meet Not Found

- Set `GST_MEET_PATH` to the absolute path of the gst-meet binary
- Or ensure gst-meet is in your PATH

### No Audio Recorded

- Ensure participants join the room and speak
- Check gst-meet logs for errors
- Verify GStreamer plugins are installed

## Development

### Running Tests

```bash
cargo test
```

### Running in Development Mode

```bash
cargo run
```

### Checking Logs

The service uses the `tracing` crate for structured logging. Set `RUST_LOG` for more verbose output:

```bash
RUST_LOG=info ./target/release/jisti-recorder
RUST_LOG=debug ./target/release/jisti-recorder
```

## License

[Add your license here]
