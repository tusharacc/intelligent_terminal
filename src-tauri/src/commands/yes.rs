use crate::state::TerminalState;

pub fn execute(
    cmd: &str,
    args: &[&str],
    stdin: Option<String>,
    st: &mut TerminalState,
) -> Result<String, String> {
    {
            let s = if args.is_empty() { "y" } else { args[0] };
            let mut result = String::new();
            for _ in 0..10 { // Just print 10 so we don't block forever and crash
                result.push_str(&format!("{}\n", s));
            }
            result.push_str("... (truncated to prevent freezing)");
            Ok(result)
        }
}
