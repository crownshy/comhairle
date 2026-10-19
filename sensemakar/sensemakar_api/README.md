# sensemakar_api

HTTP API for the `sensemakar` sensemaking library. Exposes the same analysis
tasks as `sensemakar_cli`, synchronously, over JSON — an alternative to going
through the Redis job queue (`sensemakar_jobs` / `sensemakar_workers`) when
you just want a request/response call.

Binary name: `sensemakar_api`.

## Run

```sh
cargo run -p sensemakar_api
```

### Configuration (env vars)

| Var | Default | Purpose |
|---|---|---|
| `SENSEMAKAR_API_PORT` | `8089` | Port to listen on |
| `SENSEMAKAR_OLLAMA_BASE_URL` | `http://localhost:11434` | Ollama-compatible completion API |
| `SENSEMAKAR_OLLAMA_MODEL` | `qwen3-long` | Model name |
| `SENSEMAKAR_OLLAMA_API_KEY` | _(none)_ | API key, if the server requires one |

## Endpoints

### `GET /health`

Returns `200 ok`.

### `POST /polis-report`

Runs the Polis group-description report (`WikiPollGroupDescriber`).

Request:

```json
{
  "title": "What should be the future of South Staffordshire?",
  "statements": [
    {
      "statement": "The bus never turns up on time and I end up late for work.",
      "total_votes": { "agree": 196, "disagree": 318, "pass": 115 },
      "group_votes": {
        "A": { "agree": 57, "disagree": 8, "pass": 13 },
        "B": { "agree": 12, "disagree": 204, "pass": 24 }
      }
    }
  ],
  "context": "Statements from a Polis conversation about local housing policy.",
  "additional_instructions": "Keep each group description under 3 sentences."
}
```

`context` and `additional_instructions` are optional. Response is a
`WikiPollReportResult`:

```json
{
  "group_descriptions": {
    "A": { "name": "...", "description": "...", "unique_statement": "..." }
  },
  "consensus_description": "...",
  "disagrement_description": "..."
}
```

Example:

```sh
curl -s http://localhost:8089/polis-report \
  -H 'content-type: application/json' \
  -d @poll.json
```

### `POST /generate-themes`

Runs theme extraction (`ThemeExtractor`) over a set of statements.

Request:

```json
{
  "statements": [
    { "id": "1", "speaker_id": null, "statement_type": null, "text": "The bus never turns up on time.", "lang": "en" },
    { "id": "2", "speaker_id": null, "statement_type": null, "text": "Rent has gone up so much my friends moved away.", "lang": "en" }
  ],
  "existing_themes": null,
  "allow_additional_themes": true,
  "min_themes": null,
  "max_themes": null,
  "context": "These statements are from a survey about issues in a council area.",
  "additional_instructions": null,
  "batch_size": null
}
```

Only `statements` is required; every other field has a default (see table):

| Field | Default |
|---|---|
| `existing_themes` | `null` — start from scratch |
| `allow_additional_themes` | `true` |
| `min_themes` / `max_themes` | `null` — no bound |
| `context` / `additional_instructions` | `null` |
| `batch_size` | `null` — run all statements in a single pass; set this to fold themes found in each batch into the next for large inputs |

Response is a JSON array of `Theme`:

```json
[{ "id": "1", "name": "Transport", "description": "Statements about transportation" }]
```

Example:

```sh
curl -s http://localhost:8089/generate-themes \
  -H 'content-type: application/json' \
  -d '{"statements": [{"id":"1","speaker_id":null,"statement_type":null,"text":"The bus never turns up on time.","lang":"en"}]}'
```

### `POST /assign-themes`

Runs theme assignment (`StatementThemeAssigner`): assigns each statement to
the theme it best fits.

Request:

```json
{
  "statements": [
    { "id": "1", "speaker_id": null, "statement_type": null, "text": "The bus never turns up on time.", "lang": "en" }
  ],
  "themes": [
    { "id": "1", "name": "Transport", "description": "Statements about transportation" }
  ],
  "context": "These statements are part of a survey about issues in a council area.",
  "additional_instructions": null
}
```

`statements` and `themes` are required; `context` and `additional_instructions`
are optional. Response is a `ThemeAssignmentResult`:

```json
{
  "statement_assignments": [{ "statement_id": "1", "theme_id": "1" }]
}
```

Example:

```sh
curl -s http://localhost:8089/assign-themes \
  -H 'content-type: application/json' \
  -d @assign.json
```

## Errors

A failed task returns `500` with `{ "error": "<message>" }`. There's no
input-validation layer beyond JSON deserialization — malformed or empty
`statements` reaches the model and its failure (or a confusing completion) is
what you'll see in the error message.

## Notes

- Not tied to `sensemakar_cli`'s CSV loaders — this API is JSON-in/JSON-out
  only. If you need to analyze a CSV, convert it client-side or run it
  through the CLI instead.
- There's no wikipoll-vs-statement `--format` switch here (unlike the CLI's
  `generate-themes`/`assign-themes`): `/polis-report` takes WikiPoll-shaped
  statements, the other two take `Statement`-shaped ones, by design of each
  request body.
