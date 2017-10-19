use discord::builders::{EmbedBuilder, EmbedAuthorBuilder};
use discord::model::{Message, ChannelId, ServerId};
use discord::Error as DiscordError;
use discord::ChannelRef;
use hyper::status::StatusCode;
use super::{Context, Error};
use rand;
use rand::Rng;
use regex::Regex;

pub const ERROR_COLOR: u64 = 0xff4013;
// pub const WARN_COLOR: u64 = 0xff13d2;
pub const SUCCESS_COLOR: u64 = 0x13ff40;
pub const INFO_COLOR: u64 = 0x13d2ff;

pub fn server_for(channel: ChannelId, context: &Context) -> Option<ServerId> {
    if let Some(ChannelRef::Public(server, _)) = context.state.find_channel(channel) {
        Some(server.id)
    } else { None }
}

pub fn send(message: &str, channel: ChannelId, context: &Context) -> Result<Option<Message>, Error> {
    allow_forbidden(context.discord.send_message(channel, message, &generate_nonce(), false))
}

pub fn send_incorrect_argument(position: usize, channel: ChannelId, context: &Context) -> Result<Option<Message>, Error> {
    send_embed(channel, context, |f| {
        f.description(&format!("Invalid argument given at position {}", position)).color(ERROR_COLOR)
            .author(|a| a.name("Wonderful Bot"))
    })
}

pub fn send_must_public(channel: ChannelId, context: &Context) -> Result<Option<Message>, Error> {
    send_error_embed("The command you are trying to use may only be used in a server.", channel, context)
}

pub fn send_embed<F: FnOnce(EmbedBuilder) -> EmbedBuilder>(
    channel: ChannelId,
    context: &Context,
    f: F) -> Result<Option<Message>, Error> {
    allow_forbidden(context.discord.send_embed(channel, "", f))
}

pub fn send_error_embed(message: &str, channel: ChannelId, context: &Context) -> Result<Option<Message>, Error> {
    send_embed(channel, context,
        |f| f.description(message).color(ERROR_COLOR).author(|a| build_embed_author(a, context)))
}

pub fn send_success_embed(message: &str, channel: ChannelId, context: &Context) -> Result<Option<Message>, Error> {
    send_embed(channel, context,
        |f| f.description(message).color(SUCCESS_COLOR).author(|a| build_embed_author(a, context)))
}

pub fn send_info_embed(message: &str, channel: ChannelId, context: &Context) -> Result<Option<Message>, Error> {
    send_embed(channel, context,
        |f| f.description(message).color(INFO_COLOR).author(|a| build_embed_author(a, context)))
}

// pub fn send_warn_embed(message: &str, channel: ChannelId, context: &Context) -> Result<Option<Message>, Error> {
//     send_embed(channel, context,
//         |f| f.description(message).color(WARN_COLOR).author(|a| build_embed_author(a, context)))
// }

pub fn build_embed_author(author: EmbedAuthorBuilder, context: &Context) -> EmbedAuthorBuilder {
    // For reference: including an icon here is expensive, because the library uploads the file
    // to discord.  Let's not :~)
    let user = context.state.user();
    let icon = if let &Some(ref hash) = &user.avatar {
            format!("https://cdn.discordapp.com/avatars/{}/{}.png", user.id, hash)
        } else {
            format!("https://cdn.discordapp.com/embed/avatars/{}.png", user.id.0 % 5)
        };
    author.name(&context.shard.configuration.name).icon_url(&icon)
}

pub fn parse_channel(value: &str) -> Option<&str> {
    lazy_static! { static ref CHANNEL_SYNTAX: Regex = Regex::new(r"<#([\d]+)>").unwrap(); }
    CHANNEL_SYNTAX.captures(value).and_then(|cap| cap.get(1)).map(|mat| mat.as_str())
}

pub fn parse_mention(value: &str) -> Option<&str> {
    lazy_static! { static ref MENTION_SYNTAX: Regex = Regex::new(r"<@[!&]?([\d]+)>").unwrap(); }
    MENTION_SYNTAX.captures(value).and_then(|cap| cap.get(1)).map(|mat| mat.as_str())
}

pub fn allow_forbidden<T>(result: Result<T, DiscordError>) -> Result<Option<T>, Error> {
    match result {
        Err(DiscordError::Status(status, _)) if status == StatusCode::Forbidden => {
            warn!("We were not given permissions to perform an action; this has been silently discarded.");
            Ok(None)
        },
        Err(err) => Err(err.into()),
        Ok(data) => Ok(Some(data))
    }
}

fn generate_nonce() -> String {
    rand::thread_rng().gen_ascii_chars().take(16).collect()
}
