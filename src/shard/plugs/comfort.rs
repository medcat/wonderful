use shard::Context;
use shard::plug::{Plug, PlugSet, PlugStatus, PlugResult};
use shard::util;
use error::Error;
use rand;

use discord::model::{Event, ChannelId, ServerId};

static DEFAULT_MESSAGE: &'static str = "User {user} has joined the server!";

fn join_message_channel(server: ServerId, context: &mut Context) -> Result<Option<ChannelId>, Error> {
    let id: Option<u64> = context.store
        .setting_get(server.0, "comfort.join.channel")
        .map_err(|e| -> Error { e.into() })?
        .and_then(|d: Option<String>| d)
        .and_then(|d: String| d.parse().ok());
    Ok(id.map(|v| ChannelId(v)))
}

fn join_message_value(server: ServerId, context: &mut Context) -> Result<String, Error> {
    let messages = context.store.setting_get_array(server.0, "comfort.join.message")?;
    if messages.len() > 0 {
        Ok(rand::sample(&mut rand::thread_rng(), messages, 1).remove(0))
    } else {
        Ok(DEFAULT_MESSAGE.into())
    }
}

plug! { JoinMessage => {
    fn handle_event(&self, event: &Event, context: &mut Context) -> PlugResult {
        match event {
            &Event::ServerMemberAdd(server, ref member) => {
                debug!("Found member add event!");
                if context.store.module_check_enabled(server.0, "comfort.join", false)? {
                    debug!("Module enabled, checking channel...");
                    if let Some(channel) = join_message_channel(server, context)? {
                        debug!("Found channel, running!");
                        let join_message = join_message_value(server, context)?
                            .replace("{user}", &member.user.mention().to_string());
                        util::send_info_embed(&join_message, channel, context)?;
                    }
                }
            }
            _ => {}
        }

        Ok(PlugStatus::Continue)
    }
} }

pub(super) fn init(set: &mut PlugSet) {
    set.push(JoinMessage);
}
