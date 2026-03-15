use crate::state::TerminalState;

pub fn execute(
    cmd: &str,
    args: &[&str],
    stdin: Option<String>,
    st: &mut TerminalState,
) -> Result<String, String> {
    {
            use std::net::ToSocketAddrs;
            if args.is_empty() { return Err("nslookup: missing operand".to_string()); }
            let mut result = String::new();
            match format!("{}:0", args[0]).to_socket_addrs() {
                Ok(addrs) => {
                    for addr in addrs {
                        result.push_str(&format!("Name: {}\nAddress: {}\n", args[0], addr.ip()));
                    }
                },
                Err(e) => return Err(format!("nslookup: server can't find {}: {}", args[0], e))
            }
            if !result.is_empty() { result.pop(); }
            Ok(result)
        }
}
