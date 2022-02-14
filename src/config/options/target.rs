use crate::command::RustupCommand;
use crate::{CargoMSRVError, TResult};
use once_cell::sync::OnceCell;
use std::convert::TryFrom;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Target {
    inner: InnerTarget,
}

#[derive(Debug, Clone, Eq, PartialEq)]
enum InnerTarget {
    PlatformDefault,
    Custom(String),
}

impl<T: Into<String>> From<T> for Target {
    fn from(target: T) -> Self {
        Self {
            inner: InnerTarget::Custom(target.into()),
        }
    }
}

impl Target {
    pub fn target(&self) -> impl Fn() -> TResult<String> {
        match &self.inner {
            InnerTarget::PlatformDefault => || find_platform_default_target(),
            InnerTarget::Custom(target) => || Ok(target.clone()),
        }
    }
}

fn find_platform_default_target() -> TResult<String> {
    let output = RustupCommand::new().with_stdout().show()?;

    let stdout = output.stdout();

    stdout
        .lines()
        .next()
        .ok_or(CargoMSRVError::DefaultHostTripleNotFound)
        .and_then(|line| {
            line.split_ascii_whitespace()
                .nth(2)
                .ok_or(CargoMSRVError::DefaultHostTripleNotFound)
                .map(String::from)
        })
}
