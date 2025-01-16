use dotenvy::dotenv;
use once_cell::sync::Lazy;

pub const ENV_PREFIX: &str = "CL_";

pub struct Config {
    pub login_token: String,
    pub session_secret: String,
    pub root_path: String,
    pub username: String,
    pub bind_port: String,
    pub accounts: Accounts,
}

pub struct Accounts {
    pub email: Option<String>,
    pub github: Option<String>,
    pub twitter: Option<String>,
    pub mastodon: Option<String>,
    pub discord: Option<String>,
    pub reddit: Option<String>,
}

fn load_config() -> Config {
    dotenv().expect(".env file not found");

    // return config value or panic if not set
    fn eval_required_conf(variable_name: String) -> String {
        match std::env::var(variable_name.clone()) {
            Ok(value) => {
                println!("{}: {}", variable_name, value);
                return value;
            }
            Err(_) => {
                panic!("{} not set!", variable_name)
            }
        }
    }

    // return optional value
    fn eval_optional_conf(variable_name: String, default_value: Option<&str>) -> Option<String> {
        match std::env::var(variable_name.clone()) {
            Ok(value) => {
                println!("{}: {}", variable_name, value);
                return Some(value);
            }
            Err(_) => match default_value {
                Some(val) => {
                    println!("Variable {variable_name} not set. Using default value: {val}.");
                    return Some(String::from(val));
                }
                None => {
                    println!("Variable {variable_name} not set. No default, leaving this empty.");
                    None
                }
            },
        }
    }

    fn eval_conf_var(name: &str, required_var: bool, default: Option<&str>) -> Option<String> {
        if required_var {
            Some(eval_required_conf(format!("{ENV_PREFIX}{name}")))
        } else {
            eval_optional_conf(format!("{ENV_PREFIX}{name}"), default)
        }
    }

    Config {
        login_token: eval_conf_var("SUBMIT_TOKEN", true, None).unwrap(),
        session_secret: eval_conf_var("SESSION_SECRET", true, None).unwrap(),
        root_path: eval_conf_var("ROOT_PATH", false, Some("./content")).unwrap(),
        username: eval_conf_var("USERNAME", true, None).unwrap(),
        bind_port: eval_conf_var("BIND_PORT", false, Some("8000")).unwrap(),
        accounts: Accounts {
            email: eval_conf_var("EMAIL", false, None),
            github: eval_conf_var("GITHUB_ACCOUNT", false, None),
            discord: eval_conf_var("DISCORD_ACCOUNT", false, None),
            twitter: eval_conf_var("TWITTER_ACCOUNT", false, None),
            mastodon: eval_conf_var("MASTODON_ACCOUNT", false, None),
            reddit: eval_conf_var("REDDIT_ACCOUNT", false, None),
        },
    }
}

pub static CONFIG: Lazy<Config> = Lazy::new(|| load_config());
