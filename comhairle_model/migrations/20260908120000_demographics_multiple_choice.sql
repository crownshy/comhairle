-- Derive multiple-choice options for existing data
WITH bucket_value_options AS (
    SELECT
        dq.slug,
        jsonb_build_object('value', v, 'label', bucket ->> 'label') AS option
    FROM demographics_question dq,
        jsonb_array_elements(dq.bucket_config -> 'buckets') AS bucket,
        jsonb_array_elements_text(COALESCE(bucket -> 'values', '[]'::jsonb)) AS v
    WHERE dq.response_type::text = 'string'
      AND dq.bucket_config ->> 'type' = 'text'
),
bucket_options AS (
    SELECT slug, jsonb_agg(DISTINCT option) AS options
    FROM bucket_value_options
    GROUP BY slug
),
value_options AS (
    SELECT
        dq.slug,
        jsonb_agg(DISTINCT jsonb_build_object('value', dr.value, 'label', dr.value)) AS options
    FROM demographics_question dq
    JOIN demographics_response dr ON dr.question_slug = dq.slug
    WHERE dq.response_type::text = 'string'
    GROUP BY dq.slug
)
UPDATE demographics_question dq
SET bucket_config = jsonb_build_object(
    'type', 'string',
    'options', COALESCE(bo.options, vo.options, '[]'::jsonb)
)
FROM (SELECT slug FROM demographics_question WHERE response_type::text = 'string') s
LEFT JOIN bucket_options bo ON bo.slug = s.slug
LEFT JOIN value_options vo ON vo.slug = s.slug
WHERE dq.slug = s.slug;
