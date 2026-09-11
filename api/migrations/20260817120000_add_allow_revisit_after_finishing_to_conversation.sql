-- Add the post-finishing revisit setting to the conversation table.
--
-- Defaults to true so existing conversations keep today's behaviour: before this column
-- existed there was no seal, so a step with can_revisit stayed reachable forever. With the
-- default a participant is never sealed and nothing changes until an admin turns it off.
ALTER TABLE conversation
ADD COLUMN allow_revisit_after_finishing BOOLEAN NOT NULL DEFAULT true;
