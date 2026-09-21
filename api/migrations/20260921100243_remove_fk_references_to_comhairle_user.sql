-- Remove all foreign key references to comhairle_user table to align with migration
-- to keycloak auth service

DO $$
DECLARE
    fk record;
BEGIN
    FOR fk IN
        SELECT c.conrelid::regclass AS referencing_table,
               c.conname            AS constraint_name
        FROM pg_constraint c
        WHERE c.contype = 'f'
          AND c.confrelid = 'comhairle_user'::regclass
          AND c.conparentid = 0  -- skip constraints inherited from a partitioned parent
    LOOP
        RAISE NOTICE 'Dropping % on %', fk.constraint_name, fk.referencing_table;

        EXECUTE format(
            'ALTER TABLE %s DROP CONSTRAINT %I',
            fk.referencing_table,
            fk.constraint_name
        );
    END LOOP;
END
$$;
