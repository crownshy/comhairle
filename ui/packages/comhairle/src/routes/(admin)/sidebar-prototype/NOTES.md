# Admin sidebar prototype (throwaway)

**Question:** the admin sidebar lists every conversation. What should it show instead,
especially while editing a conversation?

Five variants on the real admin layout. Flip between them with the floating bar at the
bottom (or `←` / `→`), or `?variant=<key>`. The choice sticks in a cookie across
navigation. Every variant except `current` hides the conversation tab row, since the
sidebar lists the sections instead.

| key        | idea                                                                                                                     |
| ---------- | ------------------------------------------------------------------------------------------------------------------------ |
| `current`  | today's sidebar, for comparison                                                                                          |
| `drill-in` | workspace nav; inside a conversation the sidebar swaps to its sections, with steps nested and a back link                |
| `switcher` | searchable conversation picker at the top (Cmd+K), sections grouped Build / Run / Results, workspace links at the bottom |
| `rail`     | fixed icon rail for workspace pages, plus a second panel that only appears inside a conversation (desktop only)          |
| `journey`  | the conversation shown as lifecycle stages (Set up, Launch, Engage, Learn) with done / current / locked state            |

## Verdict

_TBD: which variant, and which bits to steal from the others._

## Cleanup

Delete this folder and `src/lib/components/PrototypeSwitcher.svelte`, then revert the
`PROTOTYPE` blocks in `(admin)/+layout.svelte`, `(admin)/+layout.ts`,
`(admin)/+layout.server.ts` and `admin/conversations/[conversation_id]/+layout.svelte`.
