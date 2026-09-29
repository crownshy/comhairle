-- Add nullable contact_email to organization
ALTER TABLE organization
ADD COLUMN contact_email TEXT;
