use std::cmp::Ordering;
use std::borrow::Borrow;
use std::collections::BTreeSet;
use shard::Context;
use shard::plug::Command;
use shard::util;
use error::Error;
use discord::model::ServerId;
use super::ConfigureError;

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
// name, default
pub struct Module(&'static str, bool);

static MODULES: &'static [&'static Module] = &[
    &Module("test", false),
    &Module("utility.join", false),
    &Module("admin.log", true)
];

impl Module {
    pub fn enable(&self, server: ServerId, context: &mut Context) -> Result<(), Error> {
        if self.1 { context.store.module_clear(server.0, self.0)  }
        else { context.store.module_enable(server.0, self.0) }
    }

    pub fn disable(&self, server: ServerId, context: &mut Context) -> Result<(), Error> {
        if !self.1 { context.store.module_clear(server.0, self.0)  }
        else { context.store.module_disable(server.0, self.0) }
    }

    pub fn is_enabled(&self, server: ServerId, context: &mut Context) -> Result<bool, Error> {
        context.store.module_check_enabled(server.0, self.0, self.1)
    }
}

impl PartialEq<str> for Module { fn eq(&self, name: &str) -> bool { self.0 == name } }
impl PartialOrd<str> for Module { fn partial_cmp(&self, name: &str) -> Option<Ordering> { Some(self.0.cmp(name)) } }
impl<'a> Borrow<str> for &'a Module { fn borrow(&self) -> &str { self.0 } }

pub fn find(name: &str) -> Option<&'static Module> {
    lazy_static! {
        static ref MODULE_TREE: BTreeSet<&'static Module> = {
            let mut set = BTreeSet::new();
            for module in MODULES { set.insert(*module); }
            set
        };
    }

    MODULE_TREE.get(name).map(|m| *m)
}

pub(super) fn enable(command: &Command, context: &mut Context) -> Result<(), ConfigureError> {
    let module: &Module = command.arguments.get(1).and_then(|name| find(name))
        .ok_or(ConfigureError::InvalidArgumentError(2))?;
    let server: ServerId = util::server_for(command.message.channel_id, context)
        .ok_or(ConfigureError::NonPublicError)?;

    module.enable(server, context).map_err(|e| ConfigureError::Error(e))?;
    util::send_success_embed(&format!("Module {} was enabled.", module.0),
        command.message.channel_id, context).map(|_| ()).map_err(|e| ConfigureError::Error(e))
}

pub(super) fn disable(command: &Command, context: &mut Context) -> Result<(), ConfigureError> {
    let module: &Module = command.arguments.get(1).and_then(|name| find(name))
        .ok_or(ConfigureError::InvalidArgumentError(2))?;
    let server: ServerId = util::server_for(command.message.channel_id, context)
        .ok_or(ConfigureError::NonPublicError)?;

    module.disable(server, context).map_err(|e| ConfigureError::Error(e))?;
    util::send_success_embed(&format!("Module {} was disabled.", module.0),
        command.message.channel_id, context).map(|_| ()).map_err(|e| ConfigureError::Error(e))
}

pub(super) fn check(command: &Command, context: &mut Context) -> Result<(), ConfigureError> {
    let module: &Module = command.arguments.get(1).and_then(|name| find(name))
        .ok_or(ConfigureError::InvalidArgumentError(2))?;
    let server: ServerId = util::server_for(command.message.channel_id, context)
        .ok_or(ConfigureError::NonPublicError)?;
    let enabled = module.is_enabled(server, context).map_err(|e| ConfigureError::Error(e))?;

    if enabled {
        util::send_success_embed(&format!("Module {} is enabled.", module.0),
            command.message.channel_id, context)
    } else {
        util::send_success_embed(&format!("Module {} is disabled.", module.0),
            command.message.channel_id, context)
    }.map(|_| ()).map_err(|e| ConfigureError::Error(e))
}
