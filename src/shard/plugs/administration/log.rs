use shard::Context;
use shard::plug::{Command, PlugSet, PlugStatus};
use error::Error;
use discord::model::{ServerId, ChannelId};
use shard::plugs::configuration::module;
use shard::util;

pub fn log(server: ServerId, action: &str, context: &mut Context, options: Option<&[(&str, &str)]>) -> Result<(), Error> {
    let module = module::find("admin.log").unwrap();
    if !module.is_enabled(server, context)? { return Ok(()) }
    let channel: Option<String> = context.store.setting_get(server.0, "admin.log.channel")?;
    match channel {
        Some(channel) => {
            util::send_embed(ChannelId(channel.parse()?), context, |e| {
                e.description(action)
                    .fields(|mut e| {
                        if let Some(o) = options { for &(n, v) in o { e = e.field(n, v, false) } }
                        e
                    }).author(|a| util::build_embed_author(a, context))
            }).map(|_| ())
        }, None => Ok(())
    }
}
