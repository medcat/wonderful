use super::Error;
use super::Context;
use super::{Act, Command};
use super::util;
use super::ERROR_COLOR;
use discord::Connection;
use discord::model::{OnlineStatus, Event, Message, Channel, PublicChannel};

pub fn watch(mut conn: Connection, context: Context) -> Result<(), Error> {
    trace!("Setting the presence...");
    conn.set_presence(None, OnlineStatus::Online, false);
    loop {
        trace!("Polling for an event...");
        handle(conn.recv_event()?, &conn, &context)?;
    }
}

fn handle(event: Event, conn: &Connection, context: &Context) -> Result<(), Error> {
    trace!("Event received: {:?}", event);
    match event {
        Event::MessageCreate(message) => handle_message_create(message, conn, context),
        _ => Ok(())
    }
}

fn retrieve_prefix(channel: &PublicChannel, context: &Context) -> Result<Option<String>, Error> {
    Ok(context.store.find_prefix_for(channel.server_id.0)?)
}

fn handle_message_create(message: Message, conn: &Connection, context: &Context) -> Result<(), Error> {
    let channel = util::get_message_channel(&message, context)?;
    match &channel {
        // do nothing for now.
        &Channel::Group(_) | &Channel::Private(_) =>
            process_command(message, &context.shard.configuration.prefix, &channel, conn, context),
        &Channel::Public(ref c) => {
            let retrieved = retrieve_prefix(&c, context)?;
            let prefix = retrieved.as_ref().unwrap_or(&context.shard.configuration.prefix);
            process_command(message, prefix, &channel, conn, context)
        }
    }
}

fn process_command(
    message: Message,
    prefix: &str,
    channel: &Channel,
    conn: &Connection,
    context: &Context) -> Result<(), Error> {
    // Nope, we don't need to do anything.
    if !message.content.starts_with(prefix) { return Ok(()); }
    let name: String =
        message.content.chars().skip(prefix.len()).take_while(|c| !char::is_whitespace(*c)).collect();

    match context.shard.commands.find(&name) {
        Some(command) => run_command(command, &name, message, channel, conn, context),
        None => display_unknown_command(message, &name, channel, conn, context)
    }
}

fn display_unknown_command(
    message: Message,
    name: &str,
    channel: &Channel,
    conn: &Connection,
    context: &Context) -> Result<(), Error> {

    let id = util::get_channel_id(channel);
    context.discord.send_embed(id, "", |f| {
        f
            .title("Unknown Command")
            .description(&format!("Unknown command `{}` given.", name))
            .color(ERROR_COLOR)
            .author(|a| a.name("Wonderful Bot"))
    })?;
    Ok(())
}

fn run_command(
    command: &Command,
    name: &str,
    message: Message,
    channel: &Channel,
    conn: &Connection,
    context: &Context) -> Result<(), Error> {
    let act = Act::new(message, name, channel, conn, context);
    command.call(act)
}
