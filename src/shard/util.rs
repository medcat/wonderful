use discord::model::{Message, Channel, ChannelId};
use discord::builders::EmbedBuilder;
use discord::Error as DiscordError;
use hyper::status::StatusCode;
use super::{Context, Error};
use rand;
use rand::Rng;

pub fn get_message_channel(message: &Message, context: &Context) -> Result<Channel, Error> {
    context.discord.get_channel(message.channel_id).map_err(|e| e.into())
}

pub fn get_channel_id(channel: &Channel) -> ChannelId {
    match channel {
        &Channel::Group(ref g) => g.channel_id,
        &Channel::Private(ref p) => p.id,
        &Channel::Public(ref p) => p.id
    }
}

pub fn send(message: &str, channel: ChannelId, context: &Context) -> Result<Option<Message>, Error> {
    allow_forbidden(context.discord.send_message(channel, message, &generate_nonce(), false))
}

pub fn send_embed<F: FnOnce(EmbedBuilder) -> EmbedBuilder>(
    channel: ChannelId,
    context: &Context,
    f: F) -> Result<Option<Message>, Error> {
    allow_forbidden(context.discord.send_embed(channel, "", f))
}

fn allow_forbidden<T>(result: Result<T, DiscordError>) -> Result<Option<T>, Error> {
    match result {
        Err(DiscordError::Status(status, _)) if status == StatusCode::Forbidden => {
            warn!("We were not given permissions to perform an action; this has been silently discarded.");
            Ok(None)
        },
        Err(err) => Err(err.into()),
        Ok(data) => Ok(Some(data))
    }
}

pub fn generate_nonce() -> String {
    rand::thread_rng().gen_ascii_chars().take(16).collect()
}
