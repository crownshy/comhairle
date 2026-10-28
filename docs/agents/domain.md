# Domain docs

How the engineering skills should read this repo's domain docs. Single context: one glossary and one ADR folder for the whole repo.

## Before exploring, read these

- **`CONTEXT.md`** at the repo root: the domain glossary. Skills that mention `GLOSSARY.md` mean this file.
- **`documentation/adr/`**: read the ADRs that touch the area you're about to work in.

## Use the glossary's vocabulary

When you name a domain concept (in an issue title, a proposal, a test name), use the term as `CONTEXT.md` defines it, e.g. `Insights` is per Step and `Report` is per Conversation. If the concept you need isn't there, either you're inventing language the project doesn't use or there's a gap worth adding to `CONTEXT.md`.

## Flag ADR conflicts

If your output contradicts an existing ADR, say so explicitly rather than quietly overriding it:

> _Contradicts ADR-0012 (live report embeds), but worth reopening because..._
