-- Add context type and context ID to notifications table
ALTER TABLE notifications 
ADD COLUMN context_type TEXT NOT NULL DEFAULT 'site',
ADD COLUMN context_id UUID;