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
#[serde(rename = "configuration", default)]
pub struct Configuration {
    /// The owners of the bot.  This is only used for verifying debug commands and the like;
    /// this does not allow owners access to commands outside of debug/server commands.
    pub owners: Vec<i64>,
    /// Handling sharding.
    pub shards: Sharding,
    /// The default command prefix.
    pub prefix: String,
    /// The uri to the redis server.
    pub redis: String,
    /// The bot token.
    pub token: String
}

impl Default for Configuration {
    fn default() -> Configuration {
        Configuration {
            owners: vec![],
            shards: Sharding::default(),
            prefix: String::from("!"),
            token: String::new(),
            redis: String::from("redis://127.0.0.1/")
        }
    }
}
