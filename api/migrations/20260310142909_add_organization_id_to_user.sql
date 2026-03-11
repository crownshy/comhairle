-- Add organization_id column to comhairle_user table
ALTER TABLE comhairle_user
ADD COLUMN organization_id UUID references organization(id);
