use crate::state::TerminalState;

pub fn execute(
    cmd: &str,
    args: &[&str],
    stdin: Option<String>,
    st: &mut TerminalState,
) -> Result<String, String> {
    {
            if args.is_empty() { return Err("nmap: missing host operand".to_string()); }
            let host = args[0];
            let ports = [21, 22, 23, 25, 53, 80, 110, 111, 135, 139, 143, 443, 445, 993, 995, 1723, 3306, 3389, 5900, 8080];
            
            use std::net::{TcpStream, ToSocketAddrs};
            use std::time::Duration;
            
            let mut result = match format!("{}:0", host).to_socket_addrs() {
                Ok(mut addrs) => {
                    if let Some(addr) = addrs.next() {
                        format!("Starting Nmap (Rust)... Scan report for {} ({})\nPORT     STATE SERVICE\n", host, addr.ip())
                    } else {
                        format!("Starting Nmap (Rust)... Scan report for {}\nPORT     STATE SERVICE\n", host)
                    }
                },
                Err(_) => format!("Starting Nmap (Rust)... Scan report for {}\nPORT     STATE SERVICE\n", host)
            };
            
            let mut addrs = match format!("{}:80", host).to_socket_addrs() {
                Ok(mut a) => a.next(),
                Err(_) => None,
            };

            if let Some(mut addr) = addrs {
                for &port in &ports {
                    addr.set_port(port);
                    // Extremely aggressive timeout to speed up demonstration
                    if let Ok(_) = TcpStream::connect_timeout(&addr, Duration::from_millis(50)) {
                        result.push_str(&format!("{:<5}/tcp open\n", port));
                    }
                }
            } else {
                return Err(format!("nmap: failed to resolve host {}", host));
            }

            Ok(result)
        }
}
