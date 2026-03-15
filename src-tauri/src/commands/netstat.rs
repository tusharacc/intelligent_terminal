use crate::state::TerminalState;

pub fn execute(
    cmd: &str,
    args: &[&str],
    stdin: Option<String>,
    st: &mut TerminalState,
) -> Result<String, String> {
    {
            let mut result = String::new();
            let af_flags = netstat2::AddressFamilyFlags::IPV4 | netstat2::AddressFamilyFlags::IPV6;
            let proto_flags = netstat2::ProtocolFlags::TCP | netstat2::ProtocolFlags::UDP;
            // Using get_sockets_info
            let sockets = netstat2::get_sockets_info(af_flags, proto_flags)
                .map_err(|e| format!("netstat: error fetching sockets: {}", e))?;
                
            result.push_str("Proto  Local Address          Foreign Address        State\n");
            for s in sockets {
                match s.protocol_socket_info {
                    netstat2::ProtocolSocketInfo::Tcp(tcp_si) => {
                        result.push_str(&format!("TCP    {:<22} {:<22} {:?}\n",
                            format!("{}:{}", tcp_si.local_addr, tcp_si.local_port),
                            format!("{}:{}", tcp_si.remote_addr, tcp_si.remote_port),
                            tcp_si.state
                        ));
                    }
                    netstat2::ProtocolSocketInfo::Udp(udp_si) => {
                        result.push_str(&format!("UDP    {:<22} {:<22} \n",
                            format!("{}:{}", udp_si.local_addr, udp_si.local_port),
                            "*:*".to_string(), // UDP doesn't maintain state
                        ));
                    }
                }
            }
            // Truncate to avoid giant terminal logs temporarily
            let lines: Vec<&str> = result.lines().take(50).collect();
            if lines.len() == 50 {
                Ok(lines.join("\n") + "\n... (truncated for brevity)")
            } else {
                Ok(lines.join("\n"))
            }
        }
}
