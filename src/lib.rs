use anyhow::{Ok, Result};
use std::net::{SocketAddrV4, Ipv4Addr};
use socket2::{Domain, Protocol, Socket, Type, SockAddr};

pub fn run() -> Result<u8> {

    let mut ip_header =      vec![b"\x45\x00\x00\x54"]; // IP version | IHL | Type of service | Total length
    ip_header.extend_from_slice(&[b"\xc8\xcb\x40\x00"]); // Identification | Flags | Fragment Offset
    ip_header.extend_from_slice(&[b"\x40\x01\x3d\x20"]); // TTL | Protocol (ICMP) | Checksum
    ip_header.extend_from_slice(&[b"\x7f\x00\x00\x01"]); // SRC address
    ip_header.extend_from_slice(&[b"\x7f\x00\x00\x01"]); // DST address

    let mut icmp_header = vec![b"\x08\x00\xe5\xca"]; // Type of message, Code | Checksum
    icmp_header.extend_from_slice(&[b"\x12\x34\x00\x01"]); // Identifier | Sequence Number

    ip_header.extend(icmp_header);


    println!("{:?}", ip_header);

    // let socket = Socket::new(Domain::IPV4, Type::RAW, Some(Protocol::ICMPV4))?;

    let socket = Socket::new(Domain::IPV4, Type::RAW, Some(Protocol::TCP))?;

    let address = SockAddr::from(SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), 0));

    // socket.send_to(ip_header, &address);

    // socket.connect(SockAddr::from(_))
    Ok(0)
}
