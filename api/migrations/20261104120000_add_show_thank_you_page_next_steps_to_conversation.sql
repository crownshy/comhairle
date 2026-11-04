-- Adds a per-conversation toggle for the "What happens next" list on the thank-you page.
-- Defaults to TRUE so existing conversations keep showing the list.
ALTER TABLE conversation
ADD COLUMN show_thank_you_page_next_steps BOOLEAN NOT NULL DEFAULT true;
