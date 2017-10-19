#[macro_use]
mod plug;
mod plugs;
mod util;

pub use self::plug::*;
pub use self::plugs::init;

use super::{Configuration, Error};
use super::store::Store;
use discord::{Discord, Connection, State};

pub struct Shard {
    pub index: u8,
    pub configuration: Configuration,
    plugs: PlugSet
}

pub struct Context<'a> {
    pub shard: &'a Shard,
    pub discord: Discord,
    pub connection: Connection,
    pub store: Store,
    pub state: State,
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
        let (connection, ready) = discord.connect_sharded(self.index, self.configuration.shards.total)?;
        let state = State::new(ready);
        Ok(Context { shard: &self, discord, connection, store, state })
    }

    pub fn call(self) {
        trace!("Building context...");
        let context = self.context().unwrap_or_else(|e| ::handle_error(e));
        trace!("Beginning event loop...");
        watch(context).unwrap_or_else(|e| ::handle_error(e));
    }
}

fn watch(mut context: Context) -> Result<(), Error> {
    context.shard.plugs.trigger_start(&mut context)?;

    loop {
        debug!("Polling for an event...");
        let event = context.connection.recv_event()?;
        context.state.update(&event);
        context.shard.plugs.trigger_event(&event, &mut context)?;
    }

    // context.shard.plugs.trigger_stop(&mut context)
}
