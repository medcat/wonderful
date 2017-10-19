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
pub enum SettingKind {
    Channel, User, Role, String, Integer, Array
}

#[derive(Debug, Copy, Clone)]
// name, default
pub struct Setting(&'static str, SettingKind);

static SETTINGS: &'static [&'static Setting] = &[
    &Setting("test.channel", SettingKind::Channel),
    &Setting("test.user", SettingKind::User),
    &Setting("test.role", SettingKind::Role),
    &Setting("test.str", SettingKind::String),
    &Setting("test.int", SettingKind::Integer),
    &Setting("test.ary", SettingKind::Array),
    &Setting("comfort.join.channel", SettingKind::Channel),
    &Setting("comfort.join.message", SettingKind::Array)
];

impl Setting {
    pub fn get(&self, server: ServerId, context: &mut Context) -> Result<Option<String>, Error> {
        match self.1 {
            SettingKind::Channel =>
                Ok(context.store.setting_get(server.0, self.0)?
                    .map(|ch: String| format!("<#{}>", ch))),
            SettingKind::User =>
                Ok(context.store.setting_get(server.0, self.0)?
                    .map(|user: String| format!("<@{}>", user))),
            SettingKind::Role =>
                Ok(context.store.setting_get(server.0, self.0)?
                    .map(|role: String| format!("<@&{}>", role))),
            SettingKind::String => context.store.setting_get(server.0, self.0),
            SettingKind::Integer => context.store.setting_get(server.0, self.0),
            SettingKind::Array =>
                context.store.setting_get_array(server.0, self.0).map(|a| Some(format!("{:?}", a)))
        }
    }

    pub fn set(&self, server: ServerId, value: &str, context: &mut Context) -> Result<bool, Error> {
        match self.1 {
            SettingKind::Channel => {
                let value: Option<u64> = util::parse_channel(value).and_then(|v| v.parse().ok());
                if let Some(value) = value {
                    context.store.setting_set(server.0, self.0, value)?;
                    Ok(true)
                } else { Ok(false) }
            },
            SettingKind::User | SettingKind::Role => {
                let value: Option<u64> = util::parse_mention(value).and_then(|v| v.parse().ok());
                if let Some(value) = value {
                    context.store.setting_set(server.0, self.0, value)?;
                    Ok(true)
                } else { Ok(false) }
            },
            SettingKind::String => {
                context.store.setting_set(server.0, self.0, value)?;
                Ok(true)
            },
            SettingKind::Integer => {
                let value: Option<u64> = value.parse().ok();
                if let Some(value) = value {
                    context.store.setting_set(server.0, self.0, value)?;
                    Ok(true)
                } else { Ok(false) }
            },
            SettingKind::Array => {
                context.store.setting_replace_array(server.0, self.0, value)?;
                Ok(true)
            }
        }
    }

    pub fn push(&self, server: ServerId, value: &str, context: &mut Context) -> Result<bool, Error> {
        match self.1 {
            SettingKind::Array => context.store.setting_push_array(server.0, self.0, value).map(|_| true),
            _ => Ok(false)
        }
    }

    pub fn clear(&self, server: ServerId, context: &mut Context) -> Result<(), Error> {
        context.store.setting_clear(server.0, self.0)
    }
}

impl PartialOrd<Setting> for Setting { fn partial_cmp(&self, other: &Setting) -> Option<Ordering> { Some(self.0.cmp(other.0)) } }
impl PartialEq<Setting> for Setting { fn eq(&self, other: &Setting) -> bool { self.0 == other.0 } }
impl Eq for Setting {}
impl PartialEq<str> for Setting { fn eq(&self, name: &str) -> bool { self.0 == name } }
impl PartialOrd<str> for Setting { fn partial_cmp(&self, name: &str) -> Option<Ordering> { Some(self.0.cmp(name)) } }
impl Ord for Setting { fn cmp(&self, other: &Setting) -> Ordering { self.0.cmp(other.0) } }
impl<'a> Borrow<str> for &'a Setting { fn borrow(&self) -> &str { self.0 } }

pub fn find(name: &str) -> Option<&'static Setting> {
    lazy_static! {
        static ref SETTING_TREE: BTreeSet<&'static Setting> = {
            let mut set = BTreeSet::new();
            for setting in SETTINGS { set.insert(*setting); }
            set
        };
    }

    SETTING_TREE.get(name).map(|m| *m)
}

pub(super) fn get(command: &Command, context: &mut Context) -> Result<(), ConfigureError> {
    let setting: &Setting = command.arguments.get(1).and_then(|name| find(name))
        .ok_or(ConfigureError::InvalidArgumentError(2))?;
    let server: ServerId = util::server_for(command.message.channel_id, context)
        .ok_or(ConfigureError::NonPublicError)?;
    let value = setting.get(server, context).map_err(|e| ConfigureError::Error(e))?;

    match value {
        Some(value) => util::send_info_embed(&format!("Setting `{}` is set to `{}`.", setting.0, value),
            command.message.channel_id, context).map(|_| ()).map_err(|e| ConfigureError::Error(e)),
        None => util::send_info_embed(&format!("Setting `{}` is not set.", setting.0),
            command.message.channel_id, context).map(|_| ()).map_err(|e| ConfigureError::Error(e))
    }
}

pub(super) fn set(command: &Command, context: &mut Context) -> Result<(), ConfigureError> {
    let setting: &Setting = command.arguments.get(1).and_then(|name| find(name))
        .ok_or(ConfigureError::InvalidArgumentError(2))?;
    let server: ServerId = util::server_for(command.message.channel_id, context)
        .ok_or(ConfigureError::NonPublicError)?;
    let value = command.arguments.get(2).ok_or(ConfigureError::InvalidArgumentError(3))?;
    let success = setting.set(server, value, context).map_err(|e| ConfigureError::Error(e))?;

    if !success { return Err(ConfigureError::FormatError); }
    util::send_success_embed(&format!("Setting `{}` was set to `{}`.", setting.0, value),
        command.message.channel_id, context).map(|_| ()).map_err(|e| ConfigureError::Error(e))
}

pub(super) fn push(command: &Command, context: &mut Context) -> Result<(), ConfigureError> {
    let setting: &Setting = command.arguments.get(1).and_then(|name| find(name))
        .ok_or(ConfigureError::InvalidArgumentError(2))?;
    let server: ServerId = util::server_for(command.message.channel_id, context)
        .ok_or(ConfigureError::NonPublicError)?;
    let value = command.arguments.get(2).ok_or(ConfigureError::InvalidArgumentError(3))?;
    let success = setting.push(server, value, context).map_err(|e| ConfigureError::Error(e))?;

    if !success { return Err(ConfigureError::FormatError); }
    util::send_success_embed(&format!("Setting `{}` now has element `{}`.", setting.0, value),
        command.message.channel_id, context).map(|_| ()).map_err(|e| ConfigureError::Error(e))
}

pub(super) fn clear(command: &Command, context: &mut Context) -> Result<(), ConfigureError> {
    let setting: &Setting = command.arguments.get(1).and_then(|name| find(name))
        .ok_or(ConfigureError::InvalidArgumentError(2))?;
    let server: ServerId = util::server_for(command.message.channel_id, context)
        .ok_or(ConfigureError::NonPublicError)?;
    setting.clear(server, context).map_err(|e| ConfigureError::Error(e))?;

    util::send_success_embed(&format!("Setting `{}` was cleared.", setting.0),
        command.message.channel_id, context).map(|_| ()).map_err(|e| ConfigureError::Error(e))
}
