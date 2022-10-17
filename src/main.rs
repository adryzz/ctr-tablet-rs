use ctru::prelude::*;
use ctru::services::hid::TouchPosition;
use std::io::Write;
use std::net::{Shutdown, TcpListener, Ipv4Addr, TcpStream, ToSocketAddrs};
use std::time::Duration;

fn main() {
    ctru::init();
    let apt = Apt::init().unwrap();
    let hid = Hid::init().unwrap();
    let gfx = Gfx::init().unwrap();
    let soc = Soc::init().unwrap();
    let console = Console::init(gfx.top_screen.borrow_mut());

    let address = soc.host_address();

    let server = TcpListener::bind("0.0.0.0:5000").unwrap();
    server.set_nonblocking(true).unwrap();

    println!("IP: {address}:5000");

    while apt.main_loop() {
        // Scan for user input on the current frame.
        hid.scan_input();

        // Get information about which keys were held down on this frame
        let keys = hid.keys_held();

        let touch: (u16, u16) = TouchPosition::get(&mut TouchPosition::default());

        // We print these again because we just cleared the screen above

        for stream in server.incoming() {
            handle_client(stream.unwrap(), &touch);
        }

        if keys.intersects(KeyPad::KEY_START) {
            println!("Exiting...");
            break;
        }

        // Flush and swap framebuffers
        gfx.flush_buffers();
        gfx.swap_buffers();
        gfx.wait_for_vblank();
    }
}

fn handle_client(mut stream: TcpStream, touch: &(u16, u16)) {
    println!("Connected to {}", stream.peer_addr().unwrap());

    let mut packet: [u8; 4] = [0; 4];

    packet[0] = touch.0 as u8;
    packet[1] = (touch.0 >> 8) as u8;
    packet[2] = touch.1 as u8;
    packet[3] = (touch.1 >> 8) as u8;

    stream.write(&packet);
    stream.shutdown(Shutdown::Both).unwrap();
}
