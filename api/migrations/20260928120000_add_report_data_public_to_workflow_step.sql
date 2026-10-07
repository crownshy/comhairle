-- Opt-in for serving a step's aggregated results to the public report (ADR-0046).
-- Off by default: no step's data is public until an admin turns this on.

ALTER TABLE workflow_step
ADD COLUMN report_data_public BOOLEAN NOT NULL DEFAULT FALSE;
