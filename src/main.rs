#[repr(C, packed)]
struct Packet {
    li_vn_mode: u8,
    stratum: u8,
    poll: u8,
    root_delay: u32,
    root_dispersion: u32,
    ref_id: u32,
    ref_tm_s: u32,
    ref_tm_f: u32,
    orig_tm_s: u32,
    orig_tm_f: u32,
    rx_tm_s: u32,
    rx_tm_f: u32,
    tx_tm_s: u32,
    tx_tm_f: u32,
}

unsafe fn any_as_u8_slice<T: Sized>(p: &T) -> &[u8] {
    ::std::slice::from_raw_parts((p as *const T) as *const u8, ::std::mem::size_of::<T>())
}

fn main() -> std::io::Result<()> {
    println!("Getting time information");
    {
        let socket = std::net::UdpSocket::bind("0.0.0.0:0")?;
        let mut packet = Packet {
            li_vn_mode: 0,
            stratum: 0,
            poll: 0,
            root_delay: 0,
            root_dispersion: 0,
            ref_id: 0,
            ref_tm_s: 0,
            ref_tm_f: 0,
            orig_tm_s: 0,
            orig_tm_f: 0,
            rx_tm_s: 0,
            rx_tm_f: 0,
            tx_tm_s: 0,
            tx_tm_f: 0,
        };
        packet.li_vn_mode = 0x1b;
        socket.connect("0.0.0.0:123")?;
        let bytes: &[u8] = unsafe { any_as_u8_slice(&packet) };
        socket.send(bytes).unwrap();

        let mut buf = [0; 1024];
        match socket.peek(&mut buf) {
            Ok(received) => println!("received {} bytes", received),
            Err(e) => println!("peek function failed: {:?}", e),
        }
    }
    Ok(())
}
