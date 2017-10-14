#![allow(unused_variables)]

use std::sync::Arc;
use std::ops::{Deref, DerefMut};
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
        s.split(char::is_whitespace).map(|s| s.to_owned()).collect::<Vec<_>>())
}

pub trait Plug {
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
        let arguments = build_arguments(&message.content[name.len()..]);
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
        trace!("Trigger start!");
        self.iter().fold(Ok(PlugStatus::Continue), |last, ref plug| {
            match last { Ok(PlugStatus::Continue) => plug.handle_start(context), _ => last }
        }).map(|_| ())
    }

    pub fn trigger_event(&self, event: &Event, context: &mut Context) -> Result<(), Error> {
        trace!("Trigger event: {:?}", event);
        self.iter().fold(Ok(PlugStatus::Continue), |last, ref plug| {
            trace!("Result: {:?}", last);
            match last { Ok(PlugStatus::Continue) => plug.handle_event(event, context), _ => last }
        }).map(|_| ())
    }

    // pub fn trigger_stop(&self, context: &mut Context) -> Result<(), Error> {
    //     self.iter().fold(Ok(PlugStatus::Continue), |last, ref plug| {
    //         match last { Ok(PlugStatus::Continue) => plug.handle_stop(context), _ => last }
    //     }).map(|_| ())
    // }
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
    ($n:ident => $f:tt) => (
        #[derive(Debug, Copy, Clone, PartialEq, Eq)]
        struct $n;
        impl Plug for $n $f
    );
}
