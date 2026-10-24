mod config;
use config::Config;

const CONFIG_PATH: &str = "bot/config.toml";

fn main() {
    let config = Config::load(CONFIG_PATH)
        .expect("failed to load config. todo: proper error handling");

    println!("{:#?}", config);

    config.save(CONFIG_PATH)
        .expect("failed to save config. todo: proper error handling");
}
