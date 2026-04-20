# Live Events: Breakout Rooms & Moderator System

## Architecture Overview

### How Moderator Status Works

1. **Admin assigns facilitators** via the admin event page (`/admin/conversations/:id/events/:id` → Facilitators tab). Uses the `POST .../attendances/facilitator` endpoint with an email address.

2. **JWT route checks attendance role** — when a user hits `GET .../events/:event_id/auth`:
   - Looks up `event_attendance` for the requesting user (by user_id)
   - The Jitsi JWT `context.user.moderator` claim is set based on `attendance.role == "facilitator"`
   - The response `is_moderator` field is based on `is_user_admin` (system admin check)
   - **Note**: This means the Jitsi moderator privilege comes from the facilitator role, while the frontend `isModerator` flag (controlling breakout room UI) comes from admin status. Both paths grant moderator capabilities.
   - **Known limitation**: `get_by_user_id` doesn't filter by event_id, so if a user attends multiple events it may return the wrong attendance. A future improvement would be `get_by_user_and_event`.

3. **Jitsi respects the JWT** — when `moderator: true` in the token, Jitsi grants moderator privileges (breakout room management, mute all, lobby control, etc.)

### Breakout Room Flow

- **Auto-assign**: Gets participant list from Jitsi, round-robin distributes into groups of max 6, calls `overwriteBreakoutRooms`
- **Reshuffle**: Same as auto-assign but shuffles randomly and tries to avoid putting people in the same group as their previous assignment
- **Close**: Calls `closeBreakoutRooms` to bring everyone back to the main room
- **Inspect**: Debug tool — dumps `getRoomsInfo()` to console/alert

### Capacity Checks

Capacity only counts `participant` role attendees. Facilitators don't consume capacity slots, so you can always add facilitators even when event is "full".

---

## Removed: WebSocket Announcements

The live event page previously used WebSockets for real-time announcements. This was removed because:
- The WS broadcast system is a global broadcast, not event-scoped
- Breakout room management via Jitsi's own API is more reliable than custom WS coordination

### How to Restore WS Announcements

If you want to bring back the WS announcement feature later, here's what was removed:

#### Imports (add to `live/+page.svelte`)
```typescript
import { onMount } from 'svelte';
import { ws } from '$lib/api/websockets.svelte';
import type { WSMessage } from '$lib/api/websockets.svelte';
```

#### State variables
```typescript
let announcementText = $state('');
let announcementSending = $state(false);
let unsubWs: (() => void) | null = null;
```

#### onMount listener
```typescript
onMount(() => {
    unsubWs = ws.onMessage((msg: WSMessage) => {
        if (msg.type === 'broadcast' || msg.type === 'notification') {
            const text = msg.payload.message || msg.payload.title || 'New notification';
            showNotification(text);
        }
    });
});
```

#### Cleanup (add to onDestroy)
```typescript
unsubWs?.();
```

#### Send function
```typescript
async function sendAnnouncement() {
    if (!announcementText.trim() || announcementSending) return;
    announcementSending = true;
    try {
        await apiClient.BroadcastMessage({
            body: { message: announcementText.trim(), authenticated_only: true }
        });
        announcementText = '';
    } catch (e) {
        console.error('Failed to send announcement:', e);
    } finally {
        announcementSending = false;
    }
}
```

#### UI (add inside the `{#if isModerator}` block in Controls tab)
```svelte
<div class="space-y-2">
    <p class="text-muted-foreground text-xs font-medium tracking-wide uppercase">
        Announcements
    </p>
    <div class="flex gap-2">
        <input
            type="text"
            placeholder="Type a message for all participants..."
            bind:value={announcementText}
            onkeydown={(e) => e.key === 'Enter' && sendAnnouncement()}
            class="border-border bg-background text-foreground placeholder:text-muted-foreground focus:ring-primary flex-1 rounded-lg border px-2.5 py-1.5 text-xs focus:ring-1 focus:outline-none"
        />
        <Button
            variant="default"
            size="sm"
            class="shrink-0 text-xs"
            disabled={!announcementText.trim() || announcementSending}
            onclick={sendAnnouncement}
        >
            {announcementSending ? 'Sending...' : 'Send'}
        </Button>
    </div>
</div>
```

**Note**: The WS broadcast endpoint (`POST /ws/broadcast`) sends to ALL connected WS clients, not just those in the event. To make this event-scoped, you'd need to either:
- Use `POST /ws/broadcast/:workflow_id` with the event's conversation workflow
- Or build a new event-scoped broadcast endpoint

---

## Testing Instructions

### Prerequisites

1. Running API server with database
2. Jitsi instance configured (`PUBLIC_JITSI_DOMAIN` env var)
3. Video call service config in API (`video_call_service` in config — needs `jwt_app_id`, `jwt_app_secret`, `jwt_sub`)
4. At least 2 user accounts (one admin, one regular)

### Test 1: Facilitator Assignment (Admin UI)

1. Log in as admin
2. Go to `/admin/conversations/:id/events/:id`
3. Click the "Facilitators" tab
4. Type a registered user's email and press Enter
5. **Verify**: Badge appears with email, facilitator attendance created in DB
6. Click the X on the badge to remove
7. **Verify**: Badge removed, attendance deleted

### Test 2: Moderator Status in JWT

1. As admin, add User B as facilitator for an event (Test 1)
2. As User B, register for the event (or navigate to it)
3. Go to `/conversations/:id/events/:id/live`
4. **Verify**: The "Host" badge appears in the top bar (instead of "Attendee")
5. **Verify**: In Controls tab, the "Breakout Rooms" section is visible
6. Open browser console, check the JWT claims — `moderator` should be `true`

### Test 3: Non-Moderator User

1. As a regular user (not facilitator, not admin), register for the event
2. Go to the live page
3. **Verify**: No "Breakout Rooms" section in Controls tab
4. **Verify**: "Attendee" badge shown (in dev mode)

### Test 4: Capacity + Facilitator

1. Create event with capacity = 1
2. Register 1 participant (fills capacity)
3. Try to register another participant → should fail with "at capacity"
4. Add a facilitator via admin UI → should succeed (facilitators bypass capacity)
5. **Verify**: Event list shows `currentAttendance: 1` (only counts participants)

### Test 5: Breakout Rooms (requires 2+ participants in Jitsi)

1. Log in as facilitator, join the live event
2. Have 2+ other users join the same live event
3. In Controls tab → "Auto-assign Breakouts"
4. **Verify**: Jitsi creates breakout rooms (check Jitsi UI or "Inspect Rooms")
5. Click "Reshuffle Groups"
6. **Verify**: New room assignments created (logged to console)
7. Click "Close Breakouts"
8. **Verify**: Participants return to main room

### Test 6: Event Registration from Event Detail Page

1. As logged-in user, go to `/conversations/:id/events/:id`
2. For an upcoming event, click "Register to Attend"
3. **Verify**: Shows "✓ You're registered (participant)"
4. **Verify**: Cannot register twice (button disappears after registration)

### Test 7: Auto-Registration on Live Join

1. As a user who has NOT registered, go directly to the live page
2. **Verify**: Attendance auto-registers as "participant" when conference is joined
3. **Verify**: Console shows "Attendance registration:" warning if already registered (not an error)
