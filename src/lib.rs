use anyhow::{Ok, Result};
use clap::Parser;
use socket2::{Domain, Protocol, Socket, Type};
use std::io::Read;
use std::net::{Ipv4Addr, SocketAddrV4};
use std::str::FromStr;

#[derive(Parser)]
#[command(name = "Ring - a ping implementation in Rust")]
#[command(version = "0.0.1")]
#[command(about = "Sends ICMP echo packets to hosts", long_about = None)]
struct Cli {
    /// IPv4 address to operate on, defaults to 127.0.0.1
    address: Option<String>,

    /// Stop after <count> replies, defaults to 5
    #[arg(short, long, action = clap::ArgAction::Set)]
    count: Option<u8>,
}

struct IcmpEchoHeader {
    type_of_message: [u8; 1],
    code: [u8; 1],
    checksum: [u8; 2],
    identifier: [u8; 2],
    sequence_number: [u8; 2],
}

impl IcmpEchoHeader {
    fn to_vec(self) -> Vec<u8> {
        let mut tmp_vec = Vec::with_capacity(8);
        tmp_vec.extend_from_slice(&self.type_of_message);
        tmp_vec.extend_from_slice(&self.code);
        tmp_vec.extend_from_slice(&self.checksum);
        tmp_vec.extend_from_slice(&self.identifier);
        tmp_vec.extend_from_slice(&self.sequence_number);
        tmp_vec
    }
}

impl Default for IcmpEchoHeader {
    fn default() -> Self {
        IcmpEchoHeader {
            type_of_message: *b"\x08",
            code: *b"\x00",
            checksum: *b"\xe5\xca",
            identifier: *b"\x12\x34",
            sequence_number: *b"\x00\x01",
        }
    }
}

pub fn run() -> Result<u8> {
    let cli = Cli::parse();

    // Parse the arguments
    let request_count = match cli.count {
        Some(count) => count,
        None => 5,
    };

    let address = match cli.address {
        Some(a) => a.parse()?,
        None => Ipv4Addr::LOCALHOST,
    };

    // Build the ICMP header
    let icmp_header = IcmpEchoHeader::default().to_vec();

    // Create the socket
    let mut socket = Socket::new(Domain::IPV4, Type::RAW, Some(Protocol::ICMPV4))?;

    // Send the ICMP header
    for _ in 1..=request_count {
        // println!("Sending ICMP echo packet to {}", cli.address.as_ref().unwrap());

        socket.send_to(
            &icmp_header.as_slice(),
            &SocketAddrV4::new(address, 0).into(),
        )?;
        let mut response_buffer: [u8; 28] = [0; 28];
        socket.read(response_buffer.as_mut_slice())?;
        println!("{response_buffer:x?}");
    }

    Ok(0)
}
