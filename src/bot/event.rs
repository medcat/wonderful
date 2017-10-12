use discord::model::{Message, Event, PrivateChannel, PublicChannel, Group, Channel};
use discord::Connection;
use ::Context;
use ::bot::command::{Error, Result};

fn handle_public_message(message: &Message, channel: &PublicChannel, connection: &Connection, context: &Context) -> Result {
    let server_id = channel.server_id;
    
}

fn handle_message(message: &Message, connection: &Connection, context: &Context) -> Result {
    let channel = context.discord.get_channel(message.channel_id)?;
    match channel {
        // just ignore for now.
        Channel::Private(private) => Ok(()),
        // just ignore for now.
        Channel::Group(group) => Ok(()),
        Channel::Public(chann) => handle_public_message(message, &chann, connection, context)
    }
}

fn handle_unknown(name: &str, _: &Connection, _: &Context) -> Result {
    warn!("Unknown event {} passed!", name);
    Ok(())
}

fn handle_ignored(_: &Connection, _: &Context) -> Result {
    Ok(())
}

pub fn handle(event: &Event, connection: &Connection, context: &Context) -> Result {
    match event {
        &Event::MessageCreate(ref message) => handle_message(&message, connection, context),
        &Event::Unknown(ref name, _) => handle_unknown(&name, connection, context),
        _ => handle_ignored(connection, context)
    }
}
