use super::{Command, Context, Plug, PlugSet, PlugStatus, PlugResult};
use super::ERROR_COLOR;
use super::util;

plug!(Ping => {
    fn matches_name(&self, name: &str) -> bool { name == "ping" }
    fn handle_command(&self, command: &Command, context: &mut Context) -> PlugResult {
        util::send("pong", command.message.channel_id, context).map(|_| PlugStatus::Stop)
    }
});

plug!(Missing => {
    fn matches_name(&self, _: &str) -> bool { true }
    fn handle_command(&self, command: &Command, context: &mut Context) -> PlugResult {
        util::send_embed(command.message.channel_id, context, |f| {
            f.description(&format!("Unknown command `{}` given.", command.name))
                .color(ERROR_COLOR)
                .author(|a| a.name("Wonderful Bot"))
        }).map(|_| PlugStatus::Stop)
    }
});

pub fn init() -> PlugSet {
    let mut set = PlugSet::new();
    set.push(Ping);
    set.push(Missing);
    set
}
