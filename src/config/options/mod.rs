use crate::config::options::target::Target;
use crate::config::ConfigBuilder;
use crate::{Action, TResult};
use clap::ArgMatches;

pub mod action;
pub mod target;

pub struct NewConfig {
    shared: SharedConfig,
    subcommand_config: SubcommandConfig,
}

impl NewConfig {
    pub fn new(action: Action) -> Self {
        Self {
            shared: SharedConfig::new(action),
            subcommand_config: SubcommandConfig::None
        }
    }
}

pub struct SharedConfig {
    action: Action,
    target: Target,
}

impl SharedConfig {
    pub fn new(action: Action) -> Self {
        Self {
            action,
            target: Target::
        }
    }
}

pub enum SubcommandConfig {
    None,
}

pub struct NewConfigBuilder {
    inner: NewConfig,
}

impl NewConfigBuilder{
    pub fn new(action: Action) -> Self {
        Self {
            inner: NewConfig::new(action),
        }
    }
}

trait ProcessArgument: Default {
    fn try_process(matches: &ArgMatches, builder: &mut ConfigBuilder) -> TResult<()>;
}

impl Default for Action {
    fn default() -> Self {
        Self::Find
    }
}

impl ProcessArgument for Action {
    fn try_process(matches: &ArgMatches, builder: &mut ConfigBuilder) -> TResult<()> {
        todo!()
    }
}
