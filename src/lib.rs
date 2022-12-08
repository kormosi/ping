use anyhow::{Ok, Result};
use socket2::{Domain, Protocol, SockAddr, Socket, Type};
use std::net::{Ipv4Addr, SocketAddrV4};

pub fn run() -> Result<u8> {
    // Build IP header
    let mut ip_header = b"\x45\x00\x00\x1c".to_vec(); // IP version | IHL | Type of service | Total length
    ip_header.extend_from_slice(b"\xab\xcd\x00\x00"); // Identification | Flags | Fragment Offset
    ip_header.extend_from_slice(b"\x40\x01\xd1\x11"); // TTL | Protocol (ICMP) | Header checksum
    ip_header.extend_from_slice(b"\x7f\x00\x00\x01"); // SRC address
    ip_header.extend_from_slice(b"\x7f\x00\x00\x01"); // DST address

    // Build ICMP header
    let mut icmp_header = b"\x08\x00\xe5\xca".to_vec(); // Type of message, Code | Checksum
    icmp_header.extend_from_slice(b"\x12\x34\x00\x01"); // Identifier | Sequence Number

    // Concatenate the above
    ip_header.extend_from_slice(icmp_header.as_slice());

    // println!("{:?}", ip_header);

    let address = SockAddr::from(SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), 0));
    let socket = Socket::new(Domain::IPV4, Type::RAW, Some(Protocol::ICMPV4))?;
    socket.set_header_included(true)?;

    socket.send_to(ip_header.as_slice(), &address)?;

    Ok(0)
}
