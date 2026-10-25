use serenity::all::{Color, CreateEmbed, CreateEmbedAuthor};

pub fn error_embed(title: &str, desc: &str) -> CreateEmbed {
    CreateEmbed::new()
        .title(format!("💔 {}", title))
        .description(desc)
        .color(Color::RED)
        .author(CreateEmbedAuthor::new("MetaBot"))
}
