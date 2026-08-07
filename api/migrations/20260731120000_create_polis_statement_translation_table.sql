-- Store machine translations of Polis statements into a conversation's
-- supported languages, plus the detected/declared source language of the
-- original statement_text.

ALTER TABLE polis_statement_aux
    ADD COLUMN source_locale TEXT;

CREATE TABLE polis_statement_translation (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    polis_statement_aux_id UUID NOT NULL
        REFERENCES polis_statement_aux(id) ON DELETE CASCADE,
    locale TEXT NOT NULL,
    content TEXT NOT NULL,
    ai_generated BOOLEAN NOT NULL DEFAULT TRUE,
    requires_validation BOOLEAN NOT NULL DEFAULT TRUE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE (polis_statement_aux_id, locale)
);

CREATE INDEX idx_polis_statement_translation_aux_id
    ON polis_statement_translation(polis_statement_aux_id);
