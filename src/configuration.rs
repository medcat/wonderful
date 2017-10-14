use std::fs::{File, OpenOptions};
use std::io::{BufReader, Read, Write, ErrorKind as IoErrorKind};
use std::ops::Deref;
use toml;
use super::Error;

/// The options for configuring sharding for this server.  Sharding allows the bot to be split
/// into multiple processes/servers easily.  Keep in mind that all DMs are processed by shard
/// 0.  The configuration here allows the bot to split into multiple servers.  If, for example,
/// there are 64 shards, with 8 servers in total, the 6th server should have the following values:
/// `{ first: 40, create: 8, total: 64 }`.  First is the first shard in the server; create
/// is the number of shards on the server; and total is the total number of shards.
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename = "shards", default)]
pub struct Sharding {
    /// The first shard in the server.
    pub first: u8,
    /// The number of shards to create.
    pub create: u8,
    /// The total number of shards.
    pub total: u8
}

impl Default for Sharding {
    fn default() -> Sharding { Sharding { first: 0, create: 1, total: 1 } }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename = "bot", default)]
pub struct Bot {
    /// The owners of the bot.  This is only used for verifying debug commands and the like;
    /// this does not allow owners access to commands outside of debug/server commands.
    pub owners: Vec<i64>,
    /// The default command prefix.
    pub prefix: String,
    /// The uri to the redis server.
    pub store: String,
    /// The bot token.
    pub token: String,
    /// Handling sharding.
    pub shards: Sharding,
}

impl Default for Bot {
    fn default() -> Bot {
        Bot {
            owners: vec![],
            shards: Sharding::default(),
            prefix: String::from("!"),
            token: String::new(),
            store: String::from("redis://wonder@localhost/0")
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(default)]
struct Config { bot: Bot }

impl Default for Config {
    fn default() -> Config { Config { bot: Bot::default() } }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Configuration(String, Bot);

impl Configuration {
    pub fn from(name: &str) -> Result<Configuration, Error> {
        let name = String::from(name);
        trace!("Opening file {} for configuration...", name);
        let file = File::open(&name)?;
        let mut reader = BufReader::new(file);
        let mut contents = String::new();
        trace!("Reading configuration file...");
        reader.read_to_string(&mut contents)?;
        trace!("Loading configuration...");
        let config = toml::from_str::<Config>(&contents)?;
        Ok(Configuration(name, config.bot))
    }
}

impl Deref for Configuration {
    type Target = Bot;
    fn deref(&self) -> &Bot { &self.1 }
}

pub fn create_unless_exists(name: &str) -> Result<(), Error> {
    let open = OpenOptions::new().create_new(true).write(true).open(name);
    match open {
        Ok(mut f) => {
            let dumped =
                toml::to_string::<Config>(&Config::default())
                .map_err(|e| -> Error { e.into() })?;
            f.write_all(dumped.as_bytes())?;
            Ok(())
        },
        Err(ref e) if e.kind() == IoErrorKind::AlreadyExists => Ok(()),
        Err(e) => Err(e.into())
    }
}
