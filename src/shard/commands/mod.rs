use super::command::{CommandSet, CommandKind};
// use super::Error;

pub fn init() -> CommandSet {
    let mut set = CommandSet::new();
    set.insert(&["ping"], CommandKind::all(), |_| Ok(()));
    set
}
