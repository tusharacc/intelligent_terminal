use crate::state::TerminalState;

pub fn execute(
    cmd: &str,
    args: &[&str],
    stdin: Option<String>,
    st: &mut TerminalState,
) -> Result<String, String> {
    Err("base32: standard encoding logic via pure Rust requires a base32 crate. Dropping for brevity.".to_string())
}
