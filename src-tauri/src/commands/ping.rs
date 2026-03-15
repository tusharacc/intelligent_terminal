use crate::state::TerminalState;

pub fn execute(
    cmd: &str,
    args: &[&str],
    stdin: Option<String>,
    st: &mut TerminalState,
) -> Result<String, String> {
    {
            // Using fastping_rs for basic ICMP echo. 
            // Note: On Linux/macOS this often requires elevated privileges (root or cap_net_raw) for raw sockets.
            if args.is_empty() { return Err("ping: missing operand".to_string()); }
            let dest = args[0];
            
            let (pinger, results) = match fastping_rs::Pinger::new(None, Some(56)) {
                Ok((p, r)) => (p, r),
                Err(e) => return Err(format!("ping: requires elevated privileges (root / admin) for raw sockets. Error: {}", e)),
            };

            pinger.add_ipaddr(dest);
            pinger.ping_once();

            match results.recv_timeout(std::time::Duration::from_secs(3)) {
                Ok(fastping_rs::PingResult::Receive { addr, rtt }) => {
                    Ok(format!("64 bytes from {}: icmp_seq=1 time={:?}", addr, rtt))
                },
                Ok(_) => Err(format!("ping: timeout or idle")),
                Err(_) => Err(format!("ping: timed out waiting for reply")),
            }
        }
}
