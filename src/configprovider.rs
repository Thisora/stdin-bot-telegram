use crate::configprovider::InitializationError::{ChatIdError, TokenError};
use configparser::ini::Ini;
use std::{env, fmt};
use teloxide::prelude::ChatId;
use std::path::Path;
const CHAT_ID_VAR: &str = "CHAT_ID";
const TELOXIDE_TOKEN: &str = "TELOXIDE_TOKEN";

pub struct BotConfiguration {
    chat_id: ChatId,
    token_api: String
}

#[derive(Debug)]
pub enum InitializationError {
    ChatIdError(String),
    TokenError(String),
}

fn chat_id_env() -> Result<ChatId, InitializationError> {
    let string = env::var(CHAT_ID_VAR)
        .map_err(|_| ChatIdError(format!("Could not find env var {CHAT_ID_VAR}").to_string()))?;
    let conv = string.parse::<i64>()
        .map_err(|_| ChatIdError("Invalid value of env vars".to_string()))?;
    Ok(ChatId(conv))
}
fn token_env() -> Result<String, InitializationError> {
    let token = env::var(TELOXIDE_TOKEN)
        .map_err(|_| TokenError(format!("Could not find env var {TELOXIDE_TOKEN}").to_string()))?;
    Ok(token)
}

impl BotConfiguration {
    pub fn from_env() -> Result<BotConfiguration, InitializationError> {
        Ok(BotConfiguration {
            chat_id: chat_id_env()?,
            token_api: token_env()?,
        })
    }

    pub fn from_file(path: &Path) -> Result<BotConfiguration, InitializationError> {
        let mut config = Ini::new();
        _ = config.load(path);
        Ok(BotConfiguration {
            token_api: config
                .get("DEFAULT", "token_api")
                .ok_or(TokenError("Cound not find token_api".to_string()))?,
            chat_id: ChatId(
                config
                    .get("DEFAULT", "chat_id")
                    .ok_or(TokenError)
                    .map_err(|_| ChatIdError("Config chat_id not found".to_string().to_string()))?
                    .parse()
                    .map_err(|_| ChatIdError("Config chat_id invalid".to_string()))?,
            ),

        })
    }

    pub fn token_api(&self) -> &String {
        &self.token_api
    }

    pub fn chat_id(&self) -> &ChatId {
        &self.chat_id
    }
}

impl fmt::Display for InitializationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ChatIdError(e) => write!(f, "Error in Chat ID initialisation !\n\t{e}"),
            TokenError(e) => write!(f, "Error in Token API initialisation !\n\t{e}"),

        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST_CONFIG_PATH: &str = "res/config.ini";
    const TEST_CONFIG_PATH_WRONG: &str = "res/donoexist.ini";


    #[test]
    fn test_parsing_default_conf_ini_file() {
        let conf = BotConfiguration::from_file(Path::new(TEST_CONFIG_PATH)).unwrap();
        assert_eq!(conf.token_api, "nsiofdnfoasndiofnasiodf");
        assert_eq!(conf.chat_id.0, 10);

    }
    #[test]
    fn test_parsing_invalid_conf_ini_file() {
        let conf = BotConfiguration::from_file(Path::new(TEST_CONFIG_PATH_WRONG));
        assert_eq!(conf.is_err(), true);

    }
}


