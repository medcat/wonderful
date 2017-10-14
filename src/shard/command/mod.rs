use std::sync::Arc;
use discord::model::{Message, Channel};
use discord::Connection;
use super::{Context, Error};
use shellwords;

bitflags! {
    pub struct CommandKind: u32 {
        const PRIVATE = 0b001;
        const GROUP   = 0b010;
        const PUBLIC  = 0b100;
    }
}

pub type CommandResult = Result<(), Error>;
pub type CommandAction = Arc<Box<Fn(Act) -> CommandResult + Send + Sync + 'static>>;

#[derive(Clone)]
pub struct Act<'a> {
    pub arguments: Vec<String>,
    pub message: Message,
    pub name: String,
    pub channel: &'a Channel,
    pub connection: &'a Connection,
    pub context: &'a Context<'a>,
}

fn build_arguments(message: &Message, skip: usize) -> Vec<String> {
    let base = &message.content[skip..];
    shellwords::split(&base)
        .unwrap_or_else(|_|
            base.split(char::is_whitespace).map(|s| s.to_owned()).collect::<Vec<_>>())
}

impl<'a> Act<'a> {
    pub fn new(
        message: Message,
        name: &str,
        channel: &'a Channel,
        connection: &'a Connection,
        context: &'a Context<'a>) -> Act<'a> {
        let arguments = build_arguments(&message, name.len() + 1);
        Act { arguments, message, name: String::from(name), connection, context, channel }
    }
}

#[derive(Clone)]
pub struct Command {
    action: CommandAction,
    kind: CommandKind,
    names: Vec<&'static str>
}

impl Command {
    pub fn new(action: CommandAction, kind: CommandKind, names: &[&'static str]) -> Command {
        Command { action, kind, names: names.to_owned() }
    }

    pub fn call(&self, act: Act) -> CommandResult { (self.action)(act) }
}

#[derive(Clone)]
pub struct CommandSet(Vec<Command>);

impl CommandSet {
    pub fn new() -> CommandSet { CommandSet::default() }

    pub fn insert<T: Fn(Act) -> CommandResult + Send + Sync + 'static>(&mut self, names: &[&'static str], kind: CommandKind, action: T) {
        self.0.push(Command::new(Arc::new(Box::new(action)), kind, names));
    }

    pub fn find(&self, name: &str) -> Option<&Command> {
        self.0.iter().find(|&c| c.names.iter().any(|&n| n == name))
    }
}

impl Default for CommandSet {
    fn default() -> CommandSet { CommandSet(Vec::new()) }
}
