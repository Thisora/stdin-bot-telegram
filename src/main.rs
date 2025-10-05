mod configprovider;
mod args;

use crate::configprovider::{BotConfiguration};
use atty::Stream;
use log::{error, info, warn};
use std::path::Path;
use std::time::Duration;
use std::{env};
use clap::Parser;
use teloxide::prelude::*;
use tokio::io::AsyncReadExt;
use crate::args::Args;

/**
 TODO: Make this path configurable.
*/
const CONFIG_PATH: &str = "/etc/botsnitcher/conf.ini";
const CONFIG_PATH_ENV: &str = "CONFIG_PATH";

async fn read_from_stdin_timeout(timeout: Duration) -> String {
    if atty::is(Stream::Stdin) {
        return "No message found".to_string();
    }
    let mut buf = String::new();
    match tokio::time::timeout(timeout, tokio::io::stdin().read_to_string(&mut buf)).await {
        Ok(_) => buf,
        Err(_) => "No message found".to_string(),
    }
}

fn config_file_path() -> String {
    env::var(CONFIG_PATH_ENV).unwrap_or(CONFIG_PATH.to_string())
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    let mut builder = pretty_env_logger::formatted_builder();

    if args.quiet {
        builder.filter_level(log::LevelFilter::Error);
    } else {
        builder.filter_level(log::LevelFilter::Debug);
    }
    builder.init();


    let bot_cfg: BotConfiguration = BotConfiguration::from_env().unwrap_or_else(|error| {
        warn!("Env config invalid: {}", error);
        let path = config_file_path();
        BotConfiguration::from_file(Path::new(path.as_str())).unwrap_or_else(|err| {
            panic!("File config path invalid '{}'  error: {}", path, err);
        })
    });

    let bot = Bot::new(bot_cfg.token_api());
    let msg = read_from_stdin_timeout(Duration::from_secs(5)).await;
    match bot.send_message(bot_cfg.chat_id().clone(), msg).await {
        Ok(m) => {
            info!("{}", m.text().unwrap_or("Message send"))
        }
        Err(error) => {
            error!("{}", error)
        }
    }

    return;
}
