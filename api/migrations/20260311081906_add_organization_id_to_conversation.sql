-- Add organization_id column to conversation table
ALTER TABLE conversation
ADD COLUMN organization_id UUID REFERENCES organization(id);
