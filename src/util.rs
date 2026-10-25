use crate::config::Config;

use std::io::{self, BufRead, BufReader, Read};
use std::os::unix::process::ExitStatusExt;
use std::sync::mpsc::{self, Sender};
use std::process::{Command, Stdio};
use std::thread;

use serenity::all::{
    Color, CreateEmbed, CreateEmbedAuthor
};

#[derive(Copy, Clone, Debug)]
pub enum MessageKind {
    Stdout,
    Stderr,
}

#[derive(Debug)]
pub struct Message {
    pub kind: MessageKind,
    pub content: String,
}

#[derive(Debug)]
pub struct ExecResult {
    pub messages: Vec<Message>,
    pub exitcode: i32,
}

pub fn error_embed(title: &str, desc: &str) -> CreateEmbed {
    CreateEmbed::new()
        .author(CreateEmbedAuthor::new("MetaBot"))
        .title(format!("💔 {}", title))
        .description(desc)
        .color(Color::RED)
}

impl ExecResult {
    pub fn text(&self) -> String {
        let mut result = String::new();
        result += "```\n";
        for msg in &self.messages {
            result += &msg.content;
            result += "\n";
        }
        result += "```\n";
        result += &format!("**Exited with code {}**", self.exitcode);
        return result;
    }
}

// stolen from https://github.com/JustcordProjects/zapbox/blob/main/src/utils.rs
////////////////////////////////////////////////////////////////////////////////

fn spawn_reader<R: Read + Send + 'static>(stream: R, kind: MessageKind, sender: Sender<Message>) {
    thread::spawn(move || {
        for line in BufReader::new(stream).lines() {
            if let Ok(content) = line {
                let _ = sender.send(Message { kind, content });
            }
        }
    });
}

fn run_and_capture(cmd: &mut Command) -> io::Result<ExecResult> {
    cmd.stdout(Stdio::piped()).stderr(Stdio::piped());

    let mut child = cmd.spawn()?;
    let stdout = child.stdout.take().unwrap();
    let stderr = child.stderr.take().unwrap();

    let (sender, receiver) = mpsc::channel();

    spawn_reader(stdout, MessageKind::Stdout, sender.clone());
    spawn_reader(stderr, MessageKind::Stderr, sender.clone());

    drop(sender);

    let mut messages = Vec::new();
    for msg in receiver {
        messages.push(msg);
    }

    let status = child.wait()?;
    Ok(ExecResult {
        messages: messages,
        exitcode: status.code().unwrap_or(128 + status.signal().unwrap_or(1))
    })
}

////////////////////////////////////////////////////////////////////////////////

pub fn sh(cfg: &Config, code: &str) -> eros::Result<ExecResult> {
    let mut cmd = Command::new(&cfg.general.shell);
    cmd.arg("-c").arg(code);

    Ok(run_and_capture(&mut cmd)?)
}

pub fn jb_sh(cfg: &Config, code: &str) -> eros::Result<ExecResult> {
    assert!(cfg.container.id.is_some());

    let mut cmd = Command::new("podman");
    cmd.arg("exec")
        .arg(cfg.container.id.clone().unwrap())
        .arg(&cfg.general.shell)
        .arg("-c")
        .arg(code);

    Ok(run_and_capture(&mut cmd)?)
}
