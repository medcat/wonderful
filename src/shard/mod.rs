mod command;
mod commands;
mod event;
mod util;

use self::command::{Command, Act, CommandSet};
pub use self::commands::init as commands;

use super::{Configuration, Error};
use super::store::Store;
use discord::Discord;

pub const ERROR_COLOR: u64 = 0xff4013;

pub struct Shard {
    index: u8,
    configuration: Configuration,
    commands: CommandSet
}

pub struct Context<'a> {
    shard: &'a Shard,
    discord: Discord,
    store: Store
}

impl Shard {
    pub fn new(index: u8, configuration: Configuration, commands: CommandSet) -> Shard {
        Shard { index, configuration, commands }
    }

    fn store(&self) -> Result<Store, Error> { Store::from(&self.configuration.store) }
    fn discord(&self) -> Result<Discord, Error> {
        Discord::from_bot_token(&self.configuration.token).map_err(|e| e.into())
    }
    fn context(&self) -> Result<Context, Error> {
        Ok(Context { shard: &self, discord: self.discord()?, store: self.store()? })
    }

    pub fn call(self) {
        trace!("Building context...");
        let context = self.context().unwrap_or_else(|e| ::handle_error(e));
        trace!("Connecting via shards...");
        let conn = context.discord.connect_sharded(self.index, self.configuration.shards.total)
            .unwrap_or_else(|e| ::handle_error(e.into())).0;
        trace!("Beginning event loop...");
        event::watch(conn, context).unwrap_or_else(|e| ::handle_error(e));
    }
}
