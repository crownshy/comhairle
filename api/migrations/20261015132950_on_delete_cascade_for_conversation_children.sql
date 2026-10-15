-- Workflow-related foreign keys are cleared on conversation deletion.
ALTER TABLE event
DROP CONSTRAINT event_conversation_id_fkey;

ALTER TABLE event ADD CONSTRAINT event_conversation_id_fkey FOREIGN KEY (conversation_id) REFERENCES conversation (id) ON DELETE CASCADE;

ALTER TABLE event_attendance
DROP CONSTRAINT event_attendance_event_id_fkey;

ALTER TABLE event_attendance ADD CONSTRAINT event_attendance_event_id_fkey FOREIGN KEY (event_id) REFERENCES event (id) ON DELETE CASCADE;

ALTER TABLE feedback
DROP CONSTRAINT feedback_conversation_id_fkey;

ALTER TABLE feedback ADD CONSTRAINT feedback_conversation_id_fkey FOREIGN KEY (conversation_id) REFERENCES conversation (id) ON DELETE CASCADE;

ALTER TABLE invite
DROP CONSTRAINT invite_conversation_id_fkey;

ALTER TABLE invite ADD CONSTRAINT invite_conversation_id_fkey FOREIGN KEY (conversation_id) REFERENCES conversation (id) ON DELETE CASCADE;

ALTER TABLE invite
DROP CONSTRAINT invite_event_id_fkey;

ALTER TABLE invite ADD CONSTRAINT invite_event_id_fkey FOREIGN KEY (event_id) REFERENCES event (id) ON DELETE CASCADE;

ALTER TABLE report
DROP CONSTRAINT report_conversation_id_fkey;

ALTER TABLE report ADD CONSTRAINT report_conversation_id_fkey FOREIGN KEY (conversation_id) REFERENCES conversation (id) ON DELETE CASCADE;

ALTER TABLE report_impact
DROP CONSTRAINT report_impact_report_id_fkey;

ALTER TABLE report_impact ADD CONSTRAINT report_impact_report_id_fkey FOREIGN KEY (report_id) REFERENCES report (id) ON DELETE CASCADE;

ALTER TABLE workflow
DROP CONSTRAINT workflow_conversation_id_fkey;

ALTER TABLE workflow ADD CONSTRAINT workflow_conversation_id_fkey FOREIGN KEY (conversation_id) REFERENCES conversation (id) ON DELETE CASCADE;