use bot::Bot;
use std::env;

mod config;
mod bot;

#[tokio::main]
async fn main() -> eros::Result<()> {
    let _ = dotenvy::dotenv();

    let config_path = env::var("MB_CONFIG").unwrap_or("bot/config.toml".into());
    let bot = Bot::new(&config_path);

    bot.run().await
}
