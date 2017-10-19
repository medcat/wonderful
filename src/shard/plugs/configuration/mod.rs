use shard::Context;
use shard::plug::{Command, Plug, PlugSet, PlugStatus, PlugResult};
use shard::util;
use ::error::Error;

pub(super) mod module;
pub(super) mod setting;

// TODO: struct Module
// TODO: struct Setting (kind Channel, User, Value, Array)
// TODO: Setting.get() (works with all kinds)
// TODO: Setting.set() (works with all but Array)
// TODO: Setting.push() (works with only Array)
// TODO: Setting.clear() (works with all kinds)

#[derive(Debug)]
enum ConfigureError {
    InvalidArgumentError(usize),
    NonPublicError,
    FormatError,
    Error(Error),
}

plug! { Configure => {
    fn matches_name(&self, name: &str) -> bool { name == "configure" }
    fn handle_command(&self, command: &Command, context: &mut Context) -> PlugResult {
        let result = match command.arguments.get(0) {
            Some(&"module.enable")   => module::enable(command, context),
            Some(&"module.disable")  => module::disable(command, context),
            Some(&"module.enabled?") => module::check(command, context),
            Some(&"setting.set")     => setting::set(command, context),
            Some(&"setting.get")     => setting::get(command, context),
            Some(&"setting.clear")   => setting::clear(command, context),
            Some(&"setting.push")    => setting::push(command, context),
            _ => Err(ConfigureError::InvalidArgumentError(1))
        };

        match result {
            Err(ConfigureError::InvalidArgumentError(position)) => {
                util::send_incorrect_argument(position, command.message.channel_id, context)?;
                Ok(PlugStatus::Stop)
            },
            Err(ConfigureError::NonPublicError) => {
                util::send_must_public(command.message.channel_id, context)?;
                Ok(PlugStatus::Stop)
            },
            Err(ConfigureError::FormatError) => {
                util::send_error_embed("Incorrect format for setting value!",
                    command.message.channel_id, context)?;
                Ok(PlugStatus::Stop)
            }
            Err(ConfigureError::Error(err)) => Err(err),
            Ok(_) => Ok(PlugStatus::Stop)
        }
    }
} }

pub(super) fn init(set: &mut PlugSet) {
    set.push(Configure);
}
