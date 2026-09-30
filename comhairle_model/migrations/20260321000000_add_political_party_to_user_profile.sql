-- Add political_party column to user_profile table

ALTER TABLE user_profile
ADD COLUMN political_party TEXT DEFAULT NULL;
