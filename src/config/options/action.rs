/// The action represents the main task which the `cargo-msrv` cli should perform.
#[derive(Debug, Clone, Copy)]
pub enum Action {
    // Determines the MSRV for a project
    Find,
    // List the MSRV's as specified by package authors
    List,
    // Verifies the given MSRV
    Verify,
    // Shows the MSRV of the current crate as specified in the Cargo manifest
    Show,
}

impl From<Action> for &'static str {
    fn from(action: Action) -> Self {
        match action {
            Action::Find => "determine-msrv",
            Action::List => "list-msrv",
            Action::Verify => "verify-msrv",
            Action::Show => "show-msrv",
        }
    }
}
