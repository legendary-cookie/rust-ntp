use chrono::prelude::*;

#[repr(C, packed)]
struct Packet {
    li_vn_mode: u8,
    stratum: u8,
    poll: u8,
    precision: u8,
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
            precision: 0,
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
        let bytes: &[u8] = unsafe { any_as_u8_slice(&packet) };
        socket
            .send_to(bytes, "0.de.pool.ntp.org:123")
            .expect("Failed to send packet");
        let mut buf = [0; std::mem::size_of::<Packet>()];
        let (_amt, _src) = socket.recv_from(&mut buf)?;
        unsafe {
            let (_prefix, shorts, _suffix) = buf.align_to::<Packet>();
            let res = &shorts[0];
            let brw = std::ptr::addr_of!(res.tx_tm_s);
            let mut val = brw.read_unaligned();
            val = val.to_be() - 2208988800;
            let d = NaiveDateTime::from_timestamp(val.into(), 0);
            println!("{}", d.format("%H:%M:%S %d-%m-%Y"));
        };
    }
    Ok(())
}
