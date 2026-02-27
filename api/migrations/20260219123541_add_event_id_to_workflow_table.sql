-- Add event_id to workflow table so that workflows can be attached to events as well as conversations

ALTER TABLE workflow
ADD COLUMN event_id uuid REFERENCES event(id);
