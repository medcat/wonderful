use super::Error;
use redis::{Client, Commands};

#[derive(Debug)]
pub struct Store(Client);

impl Store {
    pub fn from(config: &str) -> Result<Store, Error> {
        Ok(Store(Client::open(config)?))
    }

    pub fn find_prefix_for(&self, server: u64) -> Result<Option<String>, Error> {
        self.0.get(format!("server:{}:prefix", server)).map_err(|e| e.into())
    }
}
