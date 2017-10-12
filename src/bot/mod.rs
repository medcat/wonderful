
use super::{Context, Configuration};
use redis::Client as RedisClient;
use discord::{Discord, Connection};
use discord::model::{Event, OnlineStatus, Message};
use std::sync::Arc;

pub mod command;
mod commands;
mod event;
pub use self::commands::init;
use self::command::{Result, Error};

fn event_loop(mut connection: Connection, context: Arc<Context>) {
    loop {
        let result = connection.recv_event().map(|event| event::handle(&event, &connection, &context));
        match result {
            Ok(_) => {},
            Err(err) => {
                error!("Error while processing event from discord: {}", err);
            }
        }
    }
}

pub fn shard(i: u8, context: Arc<Context>) {
    let total_shards = context.configuration.shards.total;
    let connection = context.discord.connect_sharded(i, total_shards);

    match connection {
        Ok((connection, _ready)) => {
            connection.set_presence(None, OnlineStatus::Online, false);
            event_loop(connection, context)
        },
        Err(err) => error!("Error attempting to connect to discord: {}", err)
    }
}
