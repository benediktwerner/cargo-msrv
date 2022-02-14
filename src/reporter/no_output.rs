use crate::config::options::action::Action;
use crate::{semver, Output, ProgressAction};

#[derive(Debug)]
pub struct NoOutput;

impl Output for NoOutput {
    fn mode(&self, _action: Action) {}
    fn set_steps(&self, _steps: u64) {}
    fn progress(&self, _action: ProgressAction) {}
    fn complete_step(&self, _version: &semver::Version, _success: bool) {}
    fn finish_success(&self, _mode: Action, _version: Option<&semver::Version>) {}
    fn finish_failure(&self, _mode: Action, _cmd: Option<&str>) {}
    fn write_line(&self, _content: &str) {}
}
