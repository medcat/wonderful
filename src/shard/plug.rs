#![allow(unused_variables)]

use std::error::Error as TraitError;
use std::sync::Arc;
use std::ops::{Deref, DerefMut};
use std::fmt::Debug;
use discord::model::{Event, Message};
use super::{Context, Error};
use shellwords;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum PlugStatus {
    Continue,
    Stop,
}

pub type PlugResult = Result<PlugStatus, Error>;

#[derive(Clone, Debug)]
pub struct Command<'a> {
    pub name: &'a str,
    pub arguments: &'a [&'a str],
    pub message: &'a Message
}

fn build_arguments(s: &str) -> Vec<String> {
    shellwords::split(&s).unwrap_or_else(|_|
        s.split(char::is_whitespace).take_while(|s| s.len() == 0).
            map(|s| s.to_owned()).collect::<Vec<_>>())
}

pub trait Plug: Debug {
    fn handle_start(&self, context: &mut Context) -> PlugResult { Ok(PlugStatus::Continue) }
    fn handle_stop(&self, context: &mut Context) -> PlugResult { Ok(PlugStatus::Continue) }
    fn matches_name(&self, &str) -> bool { false }
    fn command_prefix<'a>(&'a self, message: &Message, context: &'a mut Context) -> &'a str { &context.shard.configuration.prefix }
    fn handle_command(&self, command: &Command, context: &mut Context) -> PlugResult { Ok(PlugStatus::Continue) }
    fn handle_event(&self, event: &Event, context: &mut Context) -> PlugResult {
        match event {
            &Event::MessageCreate(ref message) => self.handle_message(&message, context),
            _ => Ok(PlugStatus::Continue)
        }
    }
    fn handle_message(&self, message: &Message, context: &mut Context) -> PlugResult {
        let prefix = {
            let prefix = self.command_prefix(message, context);
            if !message.content.starts_with(prefix) { return Ok(PlugStatus::Continue); }
            prefix.len()
        };
        let name = message.content.chars().skip(prefix)
            .take_while(|c| !char::is_whitespace(*c)).collect::<String>();
        if !self.matches_name(&name) { return Ok(PlugStatus::Continue); }
        let arguments = build_arguments(&message.content[(name.len() + 1)..]);
        let arguments = arguments.iter().map(|s| &s[..]).collect::<Vec<_>>();
        let command = Command { name: &name, arguments: &arguments[..], message };
        self.handle_command(&command, context)
    }
}

type PlugReference = Arc<Box<Plug + Send + Sync + 'static>>;

#[derive(Clone)]
pub struct PlugSet(Vec<PlugReference>);

impl PlugSet {
    pub fn new() -> PlugSet { PlugSet::default() }

    pub fn push<T: Plug + Send + Sync + 'static>(&mut self, plug: T) {
        self.0.push(Arc::new(Box::new(plug)));
    }

    pub fn trigger_start(&self, context: &mut Context) -> Result<(), Error> {
        trace!("triggering start...");
        for plug in self.iter() {
            trace!("- Plug {:?}", plug);
            match plug.handle_start(context) {
                Ok(PlugStatus::Continue) => { trace!("Continue."); }
                Ok(PlugStatus::Stop) => { trace!("Break."); break; }
                Err(err) => return Err(err.into())
            }
        }

        trace!("start trigger done.");

        Ok(())
    }

    pub fn trigger_event(&self, event: &Event, context: &mut Context) -> Result<(), Error> {
        debug!("triggering event...");
        trace!("event: {:?}", event);

        for plug in self.iter() {
            match plug.handle_event(event, context) {
                Ok(PlugStatus::Continue) => { trace!("{:?}: Continue.", plug); }
                Ok(PlugStatus::Stop) => { trace!("{:?}: Break.", plug); break; }
                Err(err) => {
                    warn!("{:?}: Error!", plug);
                    if err.is_recoverable() {
                        warn!("Error is marked as recoverable, and so will be treated as a Continue.");
                        warn!("Error: {}, {:?}", err.description(), err);
                    } else {
                        error!("Error found in plug {:?}!", plug);
                        return Err(err.into())
                    }
                }
            }
        }

        debug!("event trigger done.");
        Ok(())
    }
}

impl Default for PlugSet {
    fn default() -> PlugSet { PlugSet(Vec::new()) }
}

impl Deref for PlugSet {
    type Target = Vec<PlugReference>;
    fn deref(&self) -> &Vec<PlugReference> { &self.0 }
}

impl DerefMut for PlugSet {
    fn deref_mut(&mut self) -> &mut Vec<PlugReference> { &mut self.0 }
}

macro_rules! plug {
    () => ();

    ($n:ident => $f:tt) => (
        #[derive(Debug, Copy, Clone, PartialEq, Eq)]
        struct $n;
        impl Plug for $n $f
    );

    ($n:ident => $f:tt, $($ni:ident => $fi:tt),+) => (
        #[derive(Debug, Copy, Clone, PartialEq, Eq)]
        struct $n;
        impl Plug for $n $f

        $(
            #[derive(Debug, Copy, Clone, PartialEq, Eq)]
            struct $ni;
            impl Plug for $ni $fi
        )+
    )
}
