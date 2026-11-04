# Triage labels

The skills talk about five triage roles. This maps them to the labels we use on GitHub. We don't split "ready for an agent" from "ready for a human": both are `ready-for-dev`.

| Role in the skills | Our label         | Meaning                                          |
| ------------------ | ----------------- | ------------------------------------------------ |
| `needs-triage`     | `needs-triage`    | Someone needs to look at it and decide what next |
| `needs-info`       | `needs-info`      | Waiting on the reporter for more information     |
| `ready-for-agent`  | `ready-for-dev`   | Clear enough to build                            |
| `ready-for-human`  | `ready-for-dev`   | Clear enough to build                            |
| `wontfix`          | `wontfix`         | Will not be done                                 |

Two more labels sit outside the triage flow:

- `needs product decision`: blocked on a product call. Don't pick it up until that's made.
- `scoping`: needs shaping (spike, prototype, design) before it can be built.

When a skill names a role, apply the label from the "Our label" column. When it asks to write an agent brief for `ready-for-agent`, write it for `ready-for-dev`.
