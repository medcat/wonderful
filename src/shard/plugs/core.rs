use shard::Context;
use shard::plug::{Command, Plug, PlugSet, PlugStatus, PlugResult};
use shard::util;


plug! { Ping => {
    fn matches_name(&self, name: &str) -> bool { name == "ping" }
    fn handle_command(&self, command: &Command, context: &mut Context) -> PlugResult {
        util::send("pong", command.message.channel_id, context)?;
        Ok(PlugStatus::Stop)
    }
}, Echo => {
    fn matches_name(&self, name: &str) -> bool { name == "echo" }
    fn handle_command(&self, command: &Command, context: &mut Context) -> PlugResult {
        // let body = command.arguments.iter().map(|s| format!("'{}' ", s)).collect::<String>();
        // let body = body.replace("`", "\\`");
        util::send(&format!("{:?}", command.arguments), command.message.channel_id, context)?;
        Ok(PlugStatus::Stop)
    }
}, Missing => {
    fn matches_name(&self, _: &str) -> bool { true }
    fn handle_command(&self, command: &Command, context: &mut Context) -> PlugResult {
        info!("Command `{}' was attempted, but didn't match.", command.name);
        util::send_error_embed(&format!("Unknown command `{}`", command.name),
            command.message.channel_id, context).map(|_| PlugStatus::Stop)
    }
} }

pub(super) fn init(set: &mut PlugSet) {
    set.push(Ping);
    set.push(Echo);
    set.push(Missing);
}
