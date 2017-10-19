mod administration;
mod comfort;
mod configuration;
mod core;

use shard::plug::PlugSet;

pub fn init() -> PlugSet {
    let mut set = PlugSet::new();
    comfort::init(&mut set);
    configuration::init(&mut set);
    administration::init(&mut set);
    // utility::init(&mut set);

    // core *must* come last.
    core::init(&mut set);
    set
}
