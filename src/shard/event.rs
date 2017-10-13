use super::Error;
use super::Context;
use discord::Connection;
use discord::model::{OnlineStatus, Event, Message, Channel, PublicChannel};

pub fn watch(mut conn: Connection, context: Context) -> Result<(), Error> {
    conn.set_presence(None, OnlineStatus::Online, false);
    loop { handle(conn.recv_event()?, &conn, &context)?; }
}

fn handle(event: Event, conn: &Connection, context: &Context) -> Result<(), Error> {
    match event {
        Event::MessageCreate(message) => handle_message_create(message, conn, context),
        _ => Ok(())
    }
}

fn message_channel(message: &Message, context: &Context) -> Result<Channel, Error> {
    context.discord.get_channel(message.channel_id).map_err(|e| e.into())
}

fn retrieve_prefix(channel: &PublicChannel, context: &Context) -> Result<Option<String>, Error> {
    Ok(context.store.find_prefix_for(channel.server_id.0)?)
}

fn handle_message_create(message: Message, conn: &Connection, context: &Context) -> Result<(), Error> {
    match message_channel(&message, context)? {
        // do nothing for now.
        Channel::Group(_) | Channel::Private(_) =>
            process_command(&message, &context.shard.configuration.prefix, conn, context),
        Channel::Public(c) => {
            let retrieved = retrieve_prefix(&c, context)?;
            let prefix = retrieved.as_ref().unwrap_or(&context.shard.configuration.prefix);
            process_command(&message, prefix, conn, context)
        }
    }
}

fn process_command(message: &Message, prefix: &str, conn: &Connection, context: &Context) -> Result<(), Error> {
    unreachable!();
}
