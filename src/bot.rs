use crate::{
    cmd::{self, Command, CommandKind},
    config::Config,
    perm, util::{self, error_embed},
};

use std::path::Path;
use std::env;

use serenity::{
    all::{Context, CreateMessage, GatewayIntents, Message, Ready},
    async_trait, client::*
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

    async fn verify(&self, ctx: &Context, msg: &Message) -> bool {
        if !perm::can_execute(&self.cfg, msg) {
            let reply = CreateMessage::new()
                .embed(util::error_embed(
                    "Błąd", "Nie masz uprawnień by wykonać tą komendę."
                ))
                .reference_message(msg);

            msg.channel_id.send_message(&ctx.http, reply)
                .await.unwrap();

            return false;
        }
        return true;
    }

    pub async fn execute<'a>(&self, ctx: &Context, msg: &'a Message, cmd: Command<'a>) {
        let reply = match cmd.kind {
            CommandKind::Sh => {
                let output = util::sh(&self.cfg, cmd.code).expect("todo: proper error handling");
                CreateMessage::new().content(output.text())
            },
            CommandKind::JbSh if self.cfg.container.id.is_none() => {
                CreateMessage::new()
                    .embed(util::error_embed(
                        "Nie dostępne",
                        "ID containera nie zostało podane w konfiguracji bota więc ta funkcja jest niedostępna."
                    ))
                    .reference_message(msg)
            },
            CommandKind::JbSh => {
                let output = util::jb_sh(&self.cfg, cmd.code).expect("todo: proper error handling");
                CreateMessage::new().content(output.text())
            }
            _ => CreateMessage::new()
                .embed(util::error_embed(
                    "Nie zaimplementowano",
                    "Jeszcze nie zaimplementowane",
                ))
                .reference_message(msg)
        };

        msg.channel_id.send_message(&ctx.http, reply)
            .await
            .expect("failed to send message. todo: proper error handling");
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
            if !self.verify(&ctx, &msg).await {
                return;
            }

            if let Some(cmd) = cmd::parse(rest) {
                self.execute(&ctx, &msg, cmd).await;
                return;
            }

            match rest {
                "hello" => {
                    msg.reply(&ctx.http, "Hello, world!")
                        .await.expect("failed to send message. todo: proper error handling");
                },
                _ => {},
            }
        }
    }
}
