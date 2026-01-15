fn main(){
    // Embedding email templates
    minijinja_embed::embed_templates!("src/email_templates");
    // TODO: maybe put into one templates directory to avoid name collisions
    minijinja_embed::embed_templates!("src/agent_templates");
}
