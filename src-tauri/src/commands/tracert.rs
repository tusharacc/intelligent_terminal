use crate::state::TerminalState;

pub fn execute(
    cmd: &str,
    args: &[&str],
    stdin: Option<String>,
    st: &mut TerminalState,
) -> Result<String, String> {
    {
            use std::process::Stdio;
            if args.is_empty() { return Err(format!("{}: missing host operand", cmd)); }
            let host = args[0];
            
            let cmd_name = if cfg!(windows) { "tracert" } else { "traceroute" };
            
            match std::process::Command::new(cmd_name)
                .arg(host)
                .stdout(Stdio::piped())
                .stderr(Stdio::piped())
                .output() {
                    Ok(output) => {
                        let out_str = String::from_utf8_lossy(&output.stdout).to_string();
                        let err_str = String::from_utf8_lossy(&output.stderr).to_string();
                        if !err_str.is_empty() {
                            return Err(err_str);
                        }
                        Ok(out_str)
                    },
                    Err(e) => Err(format!("{}: failed to execute OS {} mechanism: {}", cmd, cmd_name, e))
            }
        }
}
