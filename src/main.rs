#![feature(process_exitcode_placeholder)]
use std::env;
use std::net::Ipv4Addr;
use std::process::ExitCode;
use std::time::Duration;

use net2::UdpBuilder;

fn main() -> ExitCode {
    let socket = UdpBuilder::new_v4()
        .expect("failed to crate UdpBuilder")
        .reuse_address(true)
        .expect("SO_REUSEADDR failed")
        .bind((
            Ipv4Addr::UNSPECIFIED,
            env::var("MCAST_PORT")
                .expect("MCAST_PORT env var must be set")
                .parse()
                .expect("Invalid MCAST_PORT format"),
        ))
        .expect("bind failed");
    socket
        .join_multicast_v4(
            &env::var("MCAST_IP")
                .expect("MCAST_IP env var must be set")
                .parse()
                .expect("Invalid MCAST_IP format"),
            &env::var("IFACE_IP")
                .map(|x| x.parse().expect("Invalid IFACE_IP format"))
                .unwrap_or(Ipv4Addr::UNSPECIFIED),
        )
        .expect("join_multicast failed");
    socket
        .set_nonblocking(false)
        .expect("set_nonblocking failed");
    socket
        .set_read_timeout(Some(Duration::from_secs(
            env::var("READ_TIMEOUT")
                .map(|x| x.parse().expect("Invalid READ_TIMEOUT format"))
                .unwrap_or(1),
        )))
        .expect("set_read_timeout failed");
    let mut b = [0u8];
    match socket.recv(&mut b) {
        Ok(_) => ExitCode::SUCCESS,
        Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => ExitCode::FAILURE,
        err => panic!("{:?}", err),
    }
}
