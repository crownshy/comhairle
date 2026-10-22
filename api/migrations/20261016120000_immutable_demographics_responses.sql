-- A submitted answer never changes. A user gives one answer per question version and
-- conversation (a NULL conversation is the answer kept on their profile), and later answers
-- are new rows. Reports read the answers given in their own conversation.
DROP INDEX uq_demographics_response_version_user;
CREATE UNIQUE INDEX uq_demographics_response_version_user_conversation
    ON demographics_response (question_version_id, user_id, conversation_id) NULLS NOT DISTINCT
    WHERE user_id IS NOT NULL;
CREATE INDEX idx_demographics_response_user_id ON demographics_response (user_id);

-- Anonymising (clearing user_id) is the only edit allowed. Deleting a user or a conversation
-- still clears their columns through the foreign keys.
CREATE FUNCTION demographics_response_keep_answer() RETURNS TRIGGER AS $$
BEGIN
    IF NEW.question_version_id IS DISTINCT FROM OLD.question_version_id
        OR NEW.value IS DISTINCT FROM OLD.value THEN
        RAISE EXCEPTION 'a submitted demographic response cannot be changed';
    END IF;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER trg_demographics_response_keep_answer
    BEFORE UPDATE ON demographics_response
    FOR EACH ROW EXECUTE FUNCTION demographics_response_keep_answer();
