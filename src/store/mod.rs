mod migration;
mod ukey;

use std::collections::HashMap;
use std::ops::{Deref, DerefMut};
use postgres::{Connection, TlsMode};
use postgres::params::{ConnectParams, Host};
use postgres::stmt::Statement;
use postgres::types::ToSql;
use super::configuration::{Store as StoreConfig};
use super::Error;

pub use self::migration::check as migrate;
pub use self::ukey::UKey;

const FIND_PREFIX_FOR: &'static str = r"SELECT prefix FROM wonderful_servers WHERE id = $1 LIMIT 1";
const SET_PREFIX_FOR: &'static str = r"UPDATE wonderful_servers SET prefix = $2 WHERE id = $1";

#[derive(Debug)]
pub struct Store(Connection);

impl Store {
    pub fn from(config: &StoreConfig) -> Result<Store, Error> {
        let params = match config {
                // TODO: SSL?
                &StoreConfig::Uri(ref uri) => Connection::connect(&uri[..], TlsMode::None),
                &StoreConfig::Expanded(ref location) => {
                    let mut builder = ConnectParams::builder();
                    location.port.map(|p| builder.port(p));
                    location.user.as_ref().map(|u| builder.user(u, location.pass.as_ref().map(|p| &p[..])));
                    location.base.as_ref().map(|b| builder.database(&b));
                    for (k, v) in &location.options { builder.option(&k, &v); }
                    let host =
                        if location.host.starts_with("/") { Host::Unix(location.host.clone().into()) }
                        else { Host::Tcp(location.host.clone()) };
                    Connection::connect(builder.build(host), TlsMode::None)
                }
            };
        params.map(|c| Store(c)).map_err(|e| e.into())
    }

    pub fn find_prefix_for(&self, server: UKey) -> Result<Option<String>, Error> {
        let rows = self.prepare_cached(FIND_PREFIX_FOR)?.query(&[&server])?;
        if rows.is_empty() { Ok(None) } else { Ok(rows.get(0).get("prefix")) }
    }
}

impl Deref for Store {
    type Target = Connection;
    fn deref(&self) -> &Connection { &self.0 }
}

impl DerefMut for Store { fn deref_mut(&mut self) -> &mut Connection { &mut self.0 } }
