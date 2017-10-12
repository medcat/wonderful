#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate log;
extern crate simplelog;
extern crate serde;
extern crate toml;
extern crate clap;
extern crate redis;
extern crate discord;

mod configuration;
mod bot;

use std::io::{BufReader, Read};
use std::fs::File;
use std::sync::Arc;
use clap::{Arg, App};
use configuration::Configuration;
use redis::Client as RedisClient;
use discord::Discord;
use bot::command::Set;

pub struct Context {
    pub configuration: Configuration,
    pub redis: RedisClient,
    pub discord: Discord,
    pub set: Set
}

impl Context {
    fn new(configuration: Configuration, redis: RedisClient, discord: Discord, set: Set) -> Context {
        Context { configuration, redis, discord, set }
    }
}

fn init_app<'a>() -> App<'a, 'a> {
    let app = App::new("Wonderful Bot")
        .version("0.1.0")
        .author("Jeremy Rodi <me@medcat.me>")
        .about("A bot for discord.");
    let app = app.arg(Arg::with_name("config")
        .short("c").long("config")
        .value_name("FILE")
        .help("sets a custom config file")
        .takes_value(true));
    let app = app.arg(Arg::with_name("v")
        .short("v").multiple(true).help("sets level of verbosity"));
    app
}

fn init_logging(occurrences: u64) {
    let level = match occurrences {
        1 => simplelog::LogLevelFilter::Info,
        2 => simplelog::LogLevelFilter::Debug,
        3 => simplelog::LogLevelFilter::Trace,
        _ => simplelog::LogLevelFilter::Warn
    };

    simplelog::TermLogger::init(level, simplelog::Config::default()).unwrap();
    warn!("Logging enabled.");
}

fn init_config(file: &str) -> Configuration {
    info!("Initializing configuration file...");
    let file = File::open(file).map_err(|_| ()).unwrap();
    let mut reader = BufReader::new(file);
    let mut contents = String::new();
    trace!("Reading configuration file...");
    reader.read_to_string(&mut contents).unwrap();
    trace!("Loading configuration...");
    toml::from_str::<Configuration>(&contents).unwrap()
}

fn init_redis(redis: &str) -> RedisClient {
    trace!("Creating redis client...");
    RedisClient::open(redis).unwrap()
}

fn init_discord(config: &Configuration) -> Discord {
    trace!("Creating discord client...");
    Discord::from_bot_token(&config.token).unwrap()
}

fn init_commands() -> Set {
    trace!("Loading commands...");
    bot::init()
}

fn main() {
    let app = init_app();
    let matches = app.get_matches();
    let config = matches.value_of("config").unwrap_or("config.toml");
    init_logging(matches.occurrences_of("v"));
    let configuration = init_config(config);
    let redis = init_redis(&configuration.redis);
    let discord = init_discord(&configuration);
    let commands = init_commands();
    let mut shards = vec![];
    let num_shards = configuration.shards.create;
    let context = Arc::new(Context::new(configuration, redis, discord, commands));

    for i in 0..num_shards {
        let context = context.clone();
        shards.push(std::thread::spawn(move || { bot::shard(i, context); }))
    }
}
