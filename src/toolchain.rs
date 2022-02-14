use crate::config::options::target::Target;
use once_cell::unsync::OnceCell;
use rust_releases::semver;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ToolchainSpec<'spec> {
    version: &'spec semver::Version,
    target: &'spec Target,
    spec: once_cell::unsync::OnceCell<String>,
}

impl<'spec> ToolchainSpec<'spec> {
    pub fn new(version: &'spec semver::Version, target: &'spec Target) -> Self {
        Self {
            version,
            target,
            spec: once_cell::unsync::OnceCell::new(),
        }
    }

    pub fn spec(&self) -> &str {
        self.spec
            .get_or_init(|| make_toolchain_spec(self.version, self.target.target()))
    }

    pub fn version(&self) -> &semver::Version {
        self.version
    }

    pub fn to_owned(&self) -> OwnedToolchainSpec {
        OwnedToolchainSpec {
            version: self.version.clone(),
            target: self.target.clone(),
            spec: self.spec.clone(),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OwnedToolchainSpec {
    version: semver::Version,
    target: Target,
    spec: once_cell::unsync::OnceCell<String>,
}

impl OwnedToolchainSpec {
    pub fn new(version: &semver::Version, target: Target) -> Self {
        Self {
            version: version.clone(),
            target,
            spec: OnceCell::new(),
        }
    }

    pub fn spec(&self) -> &str {
        self.spec
            .get_or_init(|| make_toolchain_spec(&self.version, self.target.target()))
    }

    pub fn version(&self) -> &semver::Version {
        &self.version
    }
}

impl std::fmt::Display for OwnedToolchainSpec {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}", self.spec()))
    }
}

fn make_toolchain_spec(version: &semver::Version, target: &str) -> String {
    format!("{}-{}", version, target)
}
