use super::{Command, Set, Action, Result};

static PING: Command = Command {
    name: "ping",
    aliases: &["pong"],
    parameters: &[],
    action: Action(&|_| Ok(()))
};

pub fn init(commands: &mut Set) {
    commands.insert(&PING);
}
