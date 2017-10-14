#[macro_use]
mod plug;
mod plugs;
mod util;

pub use self::plug::*;
pub use self::plugs::init;

use super::{Configuration, Error};
use super::store::Store;
use discord::{Discord, Connection};

pub const ERROR_COLOR: u64 = 0xff4013;

pub struct Shard {
    index: u8,
    configuration: Configuration,
    plugs: PlugSet
}

pub struct Context<'a> {
    shard: &'a Shard,
    discord: Discord,
    connection: Connection,
    store: Store,
}

impl Shard {
    pub fn new(index: u8, configuration: Configuration, plugs: PlugSet) -> Shard {
        Shard { index, configuration, plugs }
    }

    fn store(&self) -> Result<Store, Error> { Store::from(&self.configuration.store) }
    fn discord(&self) -> Result<Discord, Error> {
        Discord::from_bot_token(&self.configuration.token).map_err(|e| e.into())
    }
    fn context(&self) -> Result<Context, Error> {
        let discord = self.discord()?;
        let store = self.store()?;
        let connection = discord.connect_sharded(self.index, self.configuration.shards.total)?.0;
        Ok(Context { shard: &self, discord, store, connection })
    }

    pub fn call(self) {
        trace!("Building context...");
        let context = self.context().unwrap_or_else(|e| ::handle_error(e));
        trace!("Beginning event loop...");
        watch(context).unwrap_or_else(|e| ::handle_error(e));
    }
}

fn watch(mut context: Context) -> Result<(), Error> {
    trace!("Setting the presence...");

    context.shard.plugs.trigger_start(&mut context)?;

    loop {
        trace!("Polling for an event...");
        let event = context.connection.recv_event()?;
        context.shard.plugs.trigger_event(&event, &mut context)?;
    }

    // context.shard.plugs.trigger_stop(&mut context)
}
