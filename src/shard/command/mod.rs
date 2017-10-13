use std::sync::Arc;
use discord::model::Message;
use super::Error;

bitflags! {
    pub struct CommandKind: u32 {
        const PRIVATE = 0b001;
        const GROUP   = 0b010;
        const PUBLIC  = 0b100;
    }
}

#[derive(Clone)]
pub struct Act {
    arguments: Vec<String>,
    message: Message
}

pub type CommandResult = Result<(), Error>;
pub type CommandAction = Arc<Box<Fn(Act) -> CommandResult + Send + Sync + 'static>>;
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
}

#[derive(Clone)]
pub struct CommandSet(Vec<Command>);

impl CommandSet {
    pub fn new() -> CommandSet { CommandSet::default() }

    pub fn insert<T: Fn(Act) -> CommandResult + Send + Sync + 'static>(&mut self, names: &[&'static str], kind: CommandKind, action: T) {
        self.0.push(Command::new(Arc::new(Box::new(action)), kind, names));
    }
}

impl Default for CommandSet {
    fn default() -> CommandSet { CommandSet(Vec::new()) }
}
