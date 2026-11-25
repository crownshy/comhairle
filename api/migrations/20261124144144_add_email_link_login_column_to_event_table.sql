-- Add email_link_login column to event to control automatic login of registered users from jwt links

ALTER TABLE event
ADD COLUMN email_link_login BOOLEAN NOT NULL DEFAULT TRUE;
