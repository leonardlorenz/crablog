use std::sync::Mutex;

use dotenvy::dotenv;
use once_cell::sync::Lazy;

struct Config {
    pub submit_token: String,
    pub root_path: String,
    pub username: String,
    pub email: String,
    pub bind_port: String,
    pub accounts: Accounts
}

struct Accounts {
    pub github:    Option<String>,
    pub twitter:   Option<String>,
    pub mastodon:  Option<String>,
    pub discord:   Option<String>,
    pub reddit:    Option<String>,
}

fn load_config() -> Config {
    dotenv().expect(".env file not found");

    // return config value or panic if not set
    let eval_required_conf = |variable_name| {
        match std::env::var(variable_name) {
            Ok(_) => {
                let value = std::env::var(variable_name).unwrap();
                println!("{}: {}", variable_name, value);
                return value;
            },
            Err(_) => {
                panic!("{} not set!", variable_name)
            }
        }
    };

    // return optional value
    let eval_optional_conf = |variable_name| {
        match std::env::var(variable_name) {
            Ok(_) => {
                let value = std::env::var(variable_name).unwrap();
                println!("{}: {}", variable_name, value);
                return Some(value);
            },
            Err(_) => {
                return None
            }
        }
    };

    Config {
        submit_token: eval_required_conf("SUBMIT_TOKEN"),
        root_path: eval_required_conf("ROOT_PATH"),
        username: eval_required_conf("USERNAME"),
        email: eval_required_conf("EMAIL"),
        bind_port: eval_required_conf("BIND_PORT"),
        accounts: Accounts {
            github: eval_optional_conf("GITHUB_ACCOUNT"),
            discord: eval_optional_conf("DISCORD_ACCOUNT"),
            twitter: eval_optional_conf("TWITTER_ACCOUNT"),
            mastodon: eval_optional_conf("MASTODON_ACCOUNT"),
            reddit: eval_optional_conf("REDDIT_ACCOUNT"),
        }
    }
}

pub static CONFIG: Lazy<Config> = Lazy::new(|| {
    load_config()
});