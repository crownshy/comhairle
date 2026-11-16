# ADR-0071: PII values require explicit exposure

**Status:** accepted
**Date:** 2026-11-16

## Context

Database models and API responses sometimes contain the same personal information.
Accidentally deriving serialization or an OpenAPI schema for a database model can expose
fields that were only intended for internal use.

## Decision

Wrap protected fields in `Pii<T>`. The wrapper supports SQLx, sea-query, and
deserialization, but not serialization or `JsonSchema`. Its `Debug` output is redacted.
API responses must use a separate DTO and explicitly expose any PII they are intended to
return. The User model's email, signup IP, and signup user agent use this wrapper; the
existing user response continues to include email but not signup metadata.

## Consequences

Database values and columns remain unchanged. The protection only applies to wrapped
values: exposing or copying the inner value, manually implementing serialization or a
schema, and excluding a field from a derive are deliberate escape hatches. It does not
classify all personal information, encrypt database values, or replace authorization.
