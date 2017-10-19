#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate log;
#[macro_use]
extern crate lazy_static;
extern crate shellwords;
extern crate simplelog;
extern crate serde;
extern crate toml;
extern crate clap;
extern crate redis;
extern crate discord;
extern crate hyper;
extern crate rand;
extern crate regex;

mod configuration;
mod error;
mod shard;
pub mod store;

use std::error::Error as TraitError;
use self::error::Error;
use self::configuration::Configuration;
use self::store::Store;
use clap::{App as Application, Arg as Argument};

fn init_app<'a>() -> Application<'a, 'a> {
    let app = Application::new("Wonderful Bot")
        .version("0.1.0")
        .author("Jeremy Rodi <me@medcat.me>")
        .about("A bot for discord.");
    let app = app.arg(Argument::with_name("config")
        .short("c").long("config")
        .value_name("FILE")
        .help("sets a custom config file")
        .takes_value(true));
    let app = app.arg(Argument::with_name("v")
        .short("v").multiple(true).help("sets level of verbosity"));
    let app = app.arg(Argument::with_name("s")
        .short("s").long("suggest").help("suggests sharding configuration"));
    app
}

#[inline]
fn init_logging(occurrences: u64) {
    let level = match occurrences {
        0 => simplelog::LogLevelFilter::Warn,
        1 => simplelog::LogLevelFilter::Info,
        2 => simplelog::LogLevelFilter::Debug,
        _ => simplelog::LogLevelFilter::Trace,
    };

    simplelog::TermLogger::init(level, simplelog::Config::default()).unwrap();
    warn!("Logging enabled; level {}", level);
}

fn handle_error(e: Error) -> ! {
    error!("Could not initalize the server!  There may have been many reasons for this.");
    error!("Use the -v option to get more information.  The error message is:");
    error!("{}", e.description());
    error!("{:?}", e);
    panic!("exiting");
}

#[inline]
fn init_store(config: &Configuration) {
    trace!("Creating initial datastore connection...");
    Store::from(&config.store).unwrap_or_else(|e| handle_error(e));
}

#[inline]
fn init_config(name: &str) -> Configuration {
    configuration::create_unless_exists(name).unwrap_or_else(|e| handle_error(e));
    Configuration::from(name).unwrap_or_else(|e| handle_error(e))
}

fn handle_suggest(config: &Configuration) {
    trace!("loading discord...");
    let discord = discord::Discord::from_bot_token(&config.token)
        .unwrap_or_else(|e| handle_error(e.into()));
    trace!("retrieving suggested sharding...");
    let suggest = discord.suggested_shard_count()
        .unwrap_or_else(|e| handle_error(e.into()));
    println!("total = {}", suggest);
}

fn main() {
    let matches = init_app().get_matches();
    init_logging(matches.occurrences_of("v"));
    let config = init_config(matches.value_of("config").unwrap_or("config.toml"));

    if matches.is_present("s") { handle_suggest(&config); return; }

    let commands = shard::init();
    init_store(&config);

    (0..config.shards.create).into_iter()
        .map(|i| shard::Shard::new(i, config.clone(), commands.clone()))
        .map(|s| std::thread::spawn(move || s.call()))
        .collect::<Vec<_>>().into_iter()
        .map(|t| t.join())
        .count();
}
