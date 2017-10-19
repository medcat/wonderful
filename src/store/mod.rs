use super::Error;
use redis;
use redis::{Client, Commands, PipelineCommands};
use std::ops::Deref;

#[derive(Debug)]
pub struct Store(Client);

impl Store {
    pub fn from(config: &str) -> Result<Store, Error> {
        Ok(Store(Client::open(config)?))
    }

    // pub fn find_prefix_for(&self, server: u64) -> Result<Option<String>, Error> {
    //     self.0.get(format!("server:{}:prefix", server)).map_err(|e| e.into())
    // }

    pub fn module_enable(&self, server: u64, module: &str) -> Result<(), Error> {
        self.0.set(module_enabled_key(server, module), 1).map_err(|e| e.into())
    }

    pub fn module_disable(&self, server: u64, module: &str) -> Result<(), Error> {
        self.0.set(module_enabled_key(server, module), 0).map_err(|e| e.into())
    }

    pub fn module_clear(&self, server: u64, module: &str) -> Result<(), Error> {
        self.0.del(module_enabled_key(server, module)).map_err(|e| e.into())
    }

    pub fn module_is_enabled(&self, server: u64, module: &str) -> Result<Option<bool>, Error> {
        self.0.get(module_enabled_key(server, module))
            .map(|vopt: Option<u32>| vopt.map(|v| v != 0)).map_err(|e| -> Error { e.into() })
    }

    pub fn module_check_enabled(&self, server: u64, module: &str, default: bool) -> Result<bool, Error> {
        let result: Option<u32> = self.0.get(module_enabled_key(server, module))
            .map_err(|e| -> Error { e.into() })?;
        let result = result.map(|v| v != 0).unwrap_or(default);
        Ok(result)
    }

    pub fn setting_get<T: redis::FromRedisValue>(&self, server: u64, setting: &str) -> Result<Option<T>, Error> {
        self.0.get(setting_key(server, setting)).map_err(|e| e.into())
    }

    pub fn setting_get_array(&self, server: u64, setting: &str) -> Result<Vec<String>, Error> {
        self.0.lrange(setting_key(server, setting), 0, -1).map_err(|e| e.into())
    }

    pub fn setting_set<T: redis::ToRedisArgs>(&self, server: u64, setting: &str, value: T) -> Result<(), Error> {
        self.0.set(setting_key(server, setting), value).map_err(|e| e.into())
    }

    pub fn setting_clear(&self, server: u64, setting: &str) -> Result<(), Error> {
        self.0.del(setting_key(server, setting)).map_err(|e| e.into())
    }

    pub fn setting_replace_array<T: redis::ToRedisArgs>(&self, server: u64, setting: &str, value: T) -> Result<(), Error> {
        redis::pipe().atomic()
            .del(setting_key(server, setting)).ignore()
            .lpush(setting_key(server, setting), value).ignore()
            .query(&self.0).map_err(|e| e.into())
    }

    pub fn setting_push_array<T: redis::ToRedisArgs>(&self, server: u64, setting: &str, value: T) -> Result<(), Error> {
        self.0.rpush(setting_key(server, setting), value).map_err(|e| e.into())
    }
}

impl Deref for Store { type Target = Client; fn deref(&self) -> &Client { &self.0 } }

fn setting_key(server: u64, setting: &str) -> String {
    format!("server:{}:settings:{}", server, setting)
}

fn module_enabled_key(server: u64, module: &str) -> String {
    format!("server:{}:modules:{}:enabled", server, module)
}
