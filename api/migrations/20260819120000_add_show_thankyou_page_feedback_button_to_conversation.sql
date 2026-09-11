-- Adds a per-conversation toggle for the feedback button on the thank-you page.
-- Defaults to TRUE so existing conversations keep showing the feedback button.
ALTER TABLE conversation
ADD COLUMN show_thankyou_page_feedback_button BOOLEAN NOT NULL DEFAULT true;
