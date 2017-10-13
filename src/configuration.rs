use std::fs::File;
use std::io::BufReader;
use std::io::Read;
use std::collections::HashMap;
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
#[serde(default)]
pub struct StoreLocation {
    #[serde(rename = "database")]
    pub base: Option<String>,
    pub host: String,
    pub user: Option<String>,
    #[serde(rename = "password")]
    pub pass: Option<String>,
    pub port: Option<u16>,
    pub options: HashMap<String, String>
}

impl Default for StoreLocation {
    fn default() -> StoreLocation {
        StoreLocation {
            base: None,
            host: String::from("localhost"),
            user: None,
            pass: None,
            port: None,
            options: HashMap::new()
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum Store {
    Uri(String),
    Expanded(StoreLocation)
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename = "bot", default)]
pub struct Bot {
    /// The owners of the bot.  This is only used for verifying debug commands and the like;
    /// this does not allow owners access to commands outside of debug/server commands.
    pub owners: Vec<i64>,
    /// Handling sharding.
    pub shards: Sharding,
    /// The default command prefix.
    pub prefix: String,
    /// The uri to the redis server.
    pub store: Store,
    /// The bot token.
    pub token: String
}

impl Default for Bot {
    fn default() -> Bot {
        Bot {
            owners: vec![],
            shards: Sharding::default(),
            prefix: String::from("!"),
            token: String::new(),
            store: Store::Uri(String::from("postgres://wonder@localhost/wonder"))
        }
    }
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
        let bot = toml::from_str::<Bot>(&contents)?;
        Ok(Configuration(name, bot))
    }
}

impl Deref for Configuration {
    type Target = Bot;
    fn deref(&self) -> &Bot { &self.1 }
}
