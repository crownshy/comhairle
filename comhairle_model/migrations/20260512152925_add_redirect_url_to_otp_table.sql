-- Add redirect_url column to otp table

ALTER TABLE one_time_passcode
ADD COLUMN redirect_url TEXT NOT NULL DEFAULT '/';
