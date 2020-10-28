use netstat2::{
    get_sockets_info, AddressFamilyFlags, ProtocolFlags, ProtocolSocketInfo::Tcp,
    ProtocolSocketInfo::Udp,
};
use std::env;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();

    let argc = args.len();
    if argc != 2 {
        println!("Not 2 arguments");
        return Err("Wrong Argument".into());
    }

    match args[1].as_str() {
        "--help" => println!("Help Text"),
        _ => println!("Something else"),
    }

    let prt_char = &args[1][..1].to_uppercase();
    let (proto_flags, port) = match prt_char.as_str() {
        "T" => (ProtocolFlags::TCP, args[1][1..].to_string().parse::<u16>()?),
        "U" => (ProtocolFlags::UDP, args[1][1..].to_string().parse::<u16>()?),
        _ => (
            ProtocolFlags::TCP | ProtocolFlags::UDP,
            args[1].to_string().parse::<u16>()?,
        ),
    };

    let owner = get_port_owner(port, proto_flags)?;
    println!("Owner {}", owner);
    Ok(())
}

fn get_port_owner(
    port_num: u16,
    proto_flags: ProtocolFlags,
) -> Result<u32, Box<dyn std::error::Error>> {
    let af_flags = AddressFamilyFlags::IPV4 | AddressFamilyFlags::IPV6;
    let sockets_info = get_sockets_info(af_flags, proto_flags)?;

    for si in sockets_info {
        match si.protocol_socket_info {
            Tcp(s) => {
                if s.local_port == port_num {
                    return Ok(si.associated_pids[0]);
                }
            }
            Udp(s) => {
                if s.local_port == port_num {
                    return Ok(si.associated_pids[0]);
                }
            }
        }
    }
    Ok(0)
}
