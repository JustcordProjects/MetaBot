use crate::config::Config;
use std::path::Path;
use std::env;

use serenity::{
    async_trait,
    client::*,
    all::{Ready, Context, Message, GatewayIntents}
};

pub struct Bot {
    cfg: Config,
}

impl Bot {
    pub fn new(config_path: impl AsRef<Path>) -> Bot {
        let cfg = Config::load(&config_path)
            .expect("failed to load config. todo: proper error handling");

        cfg.save(&config_path)
            .expect("failed to save config. todo: proper error handling");

        Bot { cfg }
    }

    pub async fn run(self) -> eros::Result<()> {
        let token = env::var("MB_TOKEN")
            .expect("MB_TOKEN environment variable not set");

        let intents = GatewayIntents::GUILD_MESSAGES
            | GatewayIntents::DIRECT_MESSAGES
            | GatewayIntents::MESSAGE_CONTENT;

        let mut client = Client::builder(&token, intents)
            .event_handler(self)
            .await?;

        client.start().await?;
        Ok(())
    }
}

#[async_trait]
impl EventHandler for Bot {
    async fn ready(&self, _: Context, ready: Ready) {
        println!("Logged in as {} ({})", ready.user.name, ready.user.id);
    }

    async fn message(&self, ctx: Context, msg: Message) {
        let prefix = &self.cfg.general.prefix;
        if let Some(rest) = msg.content.strip_prefix(prefix) {
            match rest {
                "hello" => {
                    msg.channel_id.say(&ctx.http, "Hello, world!")
                        .await.expect("failed to send message. todo: proper error handling");
                },
                _ => {},
            }
        }
    }
}
