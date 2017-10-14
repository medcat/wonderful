use discord::model::{Message, Channel, ChannelId};
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

pub fn send(message: &str, channel: ChannelId, context: &Context) -> Result<Message, Error> {
    context.discord.send_message(channel, message, &generate_nonce(), false).map_err(|e| e.into())
}


pub fn generate_nonce() -> String {
    rand::thread_rng().gen_ascii_chars().take(16).collect()
}
