fn main(){
    // Embedding email templates
    minijinja_embed::embed_templates!("src/email_templates");
    // Embedding bot service agent templates
    minijinja_embed::embed_templates!("src/agent_templates");
}
