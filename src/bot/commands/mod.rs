mod common;

use super::command::Set;
use super::command::*;

pub fn init() -> Set {
    let mut commands: Set = Set::new();
    common::init(&mut commands);
    commands
}
