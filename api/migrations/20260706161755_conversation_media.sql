-- Convert image_url text column to image uuid column which references media table

ALTER TABLE conversation DROP COLUMN image_url;

ALTER TABLE conversation
ADD COLUMN image UUID REFERENCES media(id);
