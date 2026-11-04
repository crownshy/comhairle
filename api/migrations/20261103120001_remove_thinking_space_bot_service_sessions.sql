-- Thinking Space no longer uses bot_service (ragflow) agent sessions, remove
-- the rows so the context value doesn't linger with nothing left to read it.

DELETE FROM bot_service_user_session WHERE context = 'thinking_space';
