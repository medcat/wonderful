use super::command::{CommandSet, CommandKind, Act, CommandResult};
use super::util;

fn handle_ping(act: Act) -> CommandResult {
    util::send("pong", act.message.channel_id, act.context)?;
    Ok(())
}

fn handle_echo(act: Act) -> CommandResult {
    let mut s = String::new();
    for ref a in act.arguments {
        s.push_str(a);
        s.push(' ');
    }

    util::send(&s, act.message.channel_id, act.context)?;
    Ok(())
}

pub fn init() -> CommandSet {
    let mut set = CommandSet::new();
    set.insert(&["ping"], CommandKind::all(), handle_ping);
    set.insert(&["echo"], CommandKind::all(), handle_echo);
    set
}
