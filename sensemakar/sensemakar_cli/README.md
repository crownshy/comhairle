# sensemakar_cli

CLI for the `sensemakar` sensemaking library. Lets you run analysis jobs from a
terminal against a CSV/JSON file instead of going through the Redis job queue
(`sensemakar_jobs` / `sensemakar_workers`).

Binary name: `sensemakar`.

Subcommands:

- [`polis-report`](#polis-report) — describe Polis opinion groups, consensus and disagreement.
- [`generate-themes`](#generate-themes) — extract themes from a set of statements.
- [`assign-themes`](#assign-themes) — assign each statement to one of a given set of themes.

All three talk to a completion model over an Ollama-compatible API and share
the same model flags: `--model`, `--base-url`, `--api-key` (see
[Model flags](#model-flags)).

## Build / run

From the repo root:

```sh
cargo run -p sensemakar_cli -- <args>
```

Or build once and use the binary directly:

```sh
cargo build -p sensemakar_cli --release
./target/release/sensemakar <args>
```

## Model flags

Shared by every subcommand:

```
      --model <MODEL>          Ollama model name to use [default: qwen3-long]
      --base-url <BASE_URL>    Base URL of the Ollama server [default: http://localhost:11434]
      --api-key <API_KEY>      API key for the Ollama server, if required
```

The model is expected to be served over an Ollama-compatible completion API
(same setup as `sensemakar_workers`). Point `--base-url` at a different server
if you're not running Ollama locally on the default port.

## `polis-report`

Runs the Polis group-description report (`WikiPollGroupDescriber`) over a set
of Polis statements: it clusters by opinion group, then asks the model to
describe each group, the points of consensus, and the points of disagreement.

```
Usage: sensemakar polis-report [OPTIONS] --input <INPUT> --title <TITLE>

Options:
  -i, --input <INPUT>
          Path to the input data file (.csv or .json)
  -t, --title <TITLE>
          Title of the poll/conversation, used as context for the model
      --context <CONTEXT>
          Optional free-text context to prime the model with
      --additional-instructions <ADDITIONAL_INSTRUCTIONS>
          Optional additional instructions for the model
  -o, --output <OUTPUT>
          Where to write the resulting JSON report. Defaults to stdout
      --model, --base-url, --api-key
          See Model flags above
```

### Input formats

#### JSON

A JSON array of statements, matching `WikiPollStatement`:

```json
[
  {
    "statement": "The bus never turns up on time and I end up late for work.",
    "total_votes": { "agree": 196, "disagree": 318, "pass": 115 },
    "group_votes": {
      "A": { "agree": 57, "disagree": 8, "pass": 13 },
      "B": { "agree": 12, "disagree": 204, "pass": 24 }
    }
  }
]
```

See `sensemakar_core/data/wiki_poll_results.json` for a full example exported
from a real Polis conversation.

#### CSV

One row per statement. Required columns: `statement`, `agree`, `disagree`,
`pass`. Any additional columns named `<group>_agree`, `<group>_disagree`,
`<group>_pass` are read as per-group vote breakdowns — one triple of columns
per opinion group:

```csv
statement,agree,disagree,pass,A_agree,A_disagree,A_pass,B_agree,B_disagree,B_pass
"The bus never turns up on time and I end up late for work.",196,318,115,57,8,13,12,204,24
"There is nowhere safe to lock up my bike near the station.",88,201,40,30,10,5,20,80,15
```

Group columns are optional — a CSV with just `statement,agree,disagree,pass`
is valid too, it just won't have a per-group breakdown to describe.

### Examples

Run against the bundled sample data, printing the report to stdout:

```sh
cargo run -p sensemakar_cli -- polis-report \
  --input sensemakar_core/data/wiki_poll_results.json \
  --title "What should be the future of South Staffordshire?"
```

Run against a CSV export, with extra context for the model and the report
written to a file:

```sh
cargo run -p sensemakar_cli -- polis-report \
  --input statements.csv \
  --title "Housing development priorities" \
  --context "Statements from a Polis conversation about local housing policy." \
  --additional-instructions "Keep each group description under 3 sentences." \
  --output report.json
```

Point at a remote Ollama server with a different model:

```sh
cargo run -p sensemakar_cli -- polis-report \
  --input statements.csv \
  --title "Housing development priorities" \
  --base-url http://ollama.internal:11434 \
  --model llama3.1 \
  --output report.json
```

### Output

A JSON `WikiPollReportResult`:

```json
{
  "group_descriptions": {
    "A": {
      "name": "Progressive Developers",
      "description": "...",
      "unique_statement": "..."
    }
  },
  "consensus_description": "...",
  "disagrement_description": "..."
}
```

## `generate-themes`

Runs the theme extraction task (`ThemeExtractor`) over a set of statements:
asks the model to propose a set of themes (e.g. "Transport", "Housing") that
group the statements, optionally extending a pre-existing theme list.

```
Usage: sensemakar generate-themes [OPTIONS] --input <INPUT>

Options:
  -i, --input <INPUT>
          Path to the input statements file (.csv or .json)
      --format <FORMAT>
          Shape of the input file: 'statement' or 'wikipoll' [default: statement]
      --existing-themes <EXISTING_THEMES>
          Path to a file of pre-existing themes (.csv or .json) to extend rather than start from scratch
      --allow-additional-themes <ALLOW_ADDITIONAL_THEMES>
          Allow the model to propose themes beyond the existing set [default: true]
      --min-themes <MIN_THEMES>
          Minimum number of themes to extract
      --max-themes <MAX_THEMES>
          Maximum number of themes to extract
      --context <CONTEXT>
          Optional free-text context to prime the model with
      --additional-instructions <ADDITIONAL_INSTRUCTIONS>
          Optional additional instructions for the model
      --batch-size <BATCH_SIZE>
          Process statements in batches of this size, folding themes found in
          each batch into the next. Defaults to running all statements in a
          single pass
      --lang <LANG>
          BCP-47 language tag to use for CSV rows without a 'lang' column [default: en]
  -o, --output <OUTPUT>
          Where to write the resulting JSON theme list. Defaults to stdout
      --model, --base-url, --api-key
          See Model flags above
```

### Input formats

Both `generate-themes` and `assign-themes` take `--format <statement|wikipoll>`
to choose the shape of `--input` (default `statement`):

- `statement` — the generic `Statement` shape described below.
- `wikipoll` — the same statement+votes shape `polis-report` reads (CSV or
  JSON). Vote counts are ignored; each statement's position in the file
  becomes its id. Handy for running theme extraction/assignment straight off
  a Polis export without reshaping it first.

#### JSON (`--format statement`, the default)

A JSON array matching `Statement`:

```json
[
  { "id": "1", "speaker_id": null, "statement_type": null, "text": "The bus never turns up on time.", "lang": "en" },
  { "id": "2", "speaker_id": null, "statement_type": null, "text": "Rent has gone up so much my friends moved away.", "lang": "en" }
]
```

#### CSV

One row per statement. Required columns: `id`, `text`. Optional columns:
`speaker_id`, `lang` (falls back to `--lang`, default `en`, when omitted or
blank):

```csv
id,text,speaker_id,lang
1,The bus never turns up on time.,,
2,Rent has gone up so much my friends moved away.,alice,en
```

Pre-existing themes (`--existing-themes`) use the same CSV-or-JSON rule, with
a `Theme` shape instead — CSV columns `id,name,description`, or a JSON array:

```json
[{ "id": "1", "name": "Transport", "description": "Statements about transportation" }]
```

### Examples

Extract themes from scratch, printing to stdout:

```sh
cargo run -p sensemakar_cli -- generate-themes \
  --input statements.csv \
  --context "These statements are from a survey about issues in a council area."
```

Extend an existing theme list, forbidding new themes, writing to a file:

```sh
cargo run -p sensemakar_cli -- generate-themes \
  --input statements.json \
  --existing-themes themes.json \
  --allow-additional-themes=false \
  --output themes.json
```

Large input, processed in batches of 20 statements so themes compound across
batches:

```sh
cargo run -p sensemakar_cli -- generate-themes \
  --input statements.csv \
  --batch-size 20 \
  --max-themes 10 \
  --output themes.json
```

Generate themes straight off a Polis export, without converting it to the
`Statement` shape first:

```sh
cargo run -p sensemakar_cli -- generate-themes \
  --input sensemakar_core/data/wiki_poll_results.json \
  --format wikipoll \
  --output themes.json
```

### Output

A JSON array of `Theme`:

```json
[
  { "id": "1", "name": "Transport", "description": "Statements about transportation" },
  { "id": "2", "name": "Services", "description": "Statements about public services" }
]
```

## `assign-themes`

Runs the theme assignment task (`StatementThemeAssigner`) over a set of
statements and a fixed theme list: asks the model to assign each statement to
the theme it best fits.

```
Usage: sensemakar assign-themes [OPTIONS] --input <INPUT> --themes <THEMES>

Options:
  -i, --input <INPUT>
          Path to the input statements file (.csv or .json)
      --format <FORMAT>
          Shape of the input file: 'statement' or 'wikipoll' [default: statement]
  -t, --themes <THEMES>
          Path to the themes file (.csv or .json) to assign statements to
      --context <CONTEXT>
          Optional free-text context to prime the model with
      --additional-instructions <ADDITIONAL_INSTRUCTIONS>
          Optional additional instructions for the model
      --lang <LANG>
          BCP-47 language tag to use for CSV rows without a 'lang' column [default: en]
  -o, --output <OUTPUT>
          Where to write the resulting JSON theme assignments. Defaults to stdout
      --model, --base-url, --api-key
          See Model flags above
```

Input formats for `--input` and `--themes` are the same as `generate-themes`
above (`Statement`/`wikipoll` via `--format`, and `Theme`, CSV or JSON).

### Examples

```sh
cargo run -p sensemakar_cli -- assign-themes \
  --input statements.csv \
  --themes themes.json \
  --context "These statements are part of a survey about issues in a council area." \
  --output assignments.json
```

Assign themes straight off a Polis export:

```sh
cargo run -p sensemakar_cli -- assign-themes \
  --input sensemakar_core/data/wiki_poll_results.json \
  --format wikipoll \
  --themes themes.json \
  --output assignments.json
```

### Output

A JSON `ThemeAssignmentResult`: one `{ statement_id, theme_id }` pair per
input statement.

```json
{
  "statement_assignments": [
    { "statement_id": "1", "theme_id": "1" },
    { "statement_id": "2", "theme_id": "2" }
  ]
}
```
