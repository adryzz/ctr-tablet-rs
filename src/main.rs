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

    println!("ctr-tablet-rs by Lena");
    println!("IP: {address}:5000");
    println!("Connect a PC to this console...");
    println!("Press START to exit.");

    let mut connected = false;

    let mut stream_or_none: Option<TcpStream> = None;

    while apt.main_loop() {
        // Scan for user input on the current frame.
        hid.scan_input();

        // Get information about which keys were held down on this frame
        let keys = hid.keys_held();

        let touch: (u16, u16) = TouchPosition::get(&mut TouchPosition::default());

        if (connected)
        {
            // Handle touch event
            let mut arr: [u8; 4] = [0; 4];
            arr[0] = touch.0 as u8;
            arr[1] = (touch.0 >> 8) as u8;
            arr[2] = touch.1 as u8;
            arr[3] = (touch.1 >> 8) as u8;

            // Send stuff through tcp

            if let Some(stream) = &mut stream_or_none {
                stream.write(&arr);
            } else {
            println!("Disconnected from client.");
            connected = false;
            }
        }
        else 
        {
            // Accept connection if available
            if let Ok((_socket, addr)) = server.accept() {
                println!("Connected to {addr:?}");
                println!("Press B to disconnect.");
                stream_or_none = Some(_socket);
                connected = true;
            }
        }



 
        if keys.intersects(KeyPad::KEY_START) {
            println!("Exiting...");
            if (connected)
            {
                if let Some(stream) = &stream_or_none {
                    stream.shutdown(Shutdown::Both);
                }
                connected = false;
            }
            break;
        }
        else if keys.intersects(KeyPad::KEY_B) {
            if (connected)
            {
                println!("Disconnecting from client...");
                if let Some(stream) = &stream_or_none {
                    stream.shutdown(Shutdown::Both);
                }
                connected = false;
            }
        }

        // Flush and swap framebuffers
        gfx.flush_buffers();
        gfx.swap_buffers();
        gfx.wait_for_vblank();
    }
}
