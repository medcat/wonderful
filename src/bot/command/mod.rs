mod set;

pub use self::set::Set;
use super::Context;
use std;
use discord;

#[derive(Debug)]
pub enum Error {
    InvalidSyntaxError,
    UntargetableError,
    DiscordError(discord::Error),
    UnknownError
}

impl std::convert::From<discord::Error> for Error {
    fn from(err: discord::Error) -> Error { Error::DiscordError(err) }
}

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum Parameter {
    String,
    User,
    Channel,
    Integer
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum Argument<'a> {
    String(&'a str),
    User(discord::model::UserId),
    Channel(discord::model::ChannelId),
    Integer(i64)
}

#[derive(Clone)]
pub struct Act<'a> {
    message: &'a discord::model::Message,
    arguments: &'a [Argument<'a>],
    connection: &'a discord::Connection,
    context: &'a Context
}

pub type Result = std::result::Result<(), Error>;
pub struct Action<'a>(pub &'a Fn(Act) -> Result);
#[derive(Clone)]
pub struct Command<'a> {
    pub name: &'a str,
    pub aliases: &'a [&'a str],
    pub parameters: &'a [Parameter],
    pub action: Action<'a>
}

unsafe impl<'a> Sync for Action<'a> {}
impl<'a> Clone for Action<'a> {
    fn clone(&self) -> Self { Action(self.0) }
}
