-- Convert conversation table to use media fields

ALTER TABLE conversation
ADD COLUMN image uuid REFERENCES media(id),
DROP COLUMN image_url,
ADD COLUMN video uuid REFERENCES media(id),
DROP COLUMN video_url;
