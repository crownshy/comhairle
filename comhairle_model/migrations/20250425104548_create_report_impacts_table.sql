CREATE TABLE REPORT_IMPACT(
    id Uuid PRIMARY KEY DEFAULT uuid_generate_v4(),
    created_by Uuid REFERENCES comhairle_user,
    report_id Uuid REFERENCES report,
    title Text,
    details Text,
    kind Text,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
)
