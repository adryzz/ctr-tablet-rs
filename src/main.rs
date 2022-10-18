use ctru::prelude::*;
use ctru::services::hid::TouchPosition;
use std::io::Write;
use std::net::{Ipv4Addr, Shutdown, TcpListener, TcpStream};

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

    setup(address);

    let mut connected = false;

    let mut stream_or_none: Option<TcpStream> = None;

    let mut calibration_step = 0;

    let mut last_keys = KeyPad::empty();

    while apt.main_loop() {
        // Scan for user input on the current frame.
        hid.scan_input();

        // Get information about which keys were held down on this frame
        let keys = hid.keys_held();

        let touch: (u16, u16) = TouchPosition::get(&mut TouchPosition::default());

        if connected {
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
                console.clear();
                setup(address);
            }
        } else {
            // Accept connection if available
            if let Ok((_socket, addr)) = server.accept() {
                println!("Connected to {addr:?}");

                if let Err(e) = _socket.set_nodelay(true)
                {
                    println!("Failed to set TCP_NODELAY. Expect higher latencies.")
                }
                println!("Press B to disconnect.");
                stream_or_none = Some(_socket);
                connected = true;
            }
        }

        if keys != last_keys {
            if keys.intersects(KeyPad::KEY_START) {
                println!("Exiting...");
                if connected {
                    if let Some(stream) = &stream_or_none {
                        stream.shutdown(Shutdown::Both);
                    }
                    connected = false;
                }
                break;
            } else if keys.intersects(KeyPad::KEY_B) {
                if connected {
                    console.clear();
                    println!("Disconnecting from client...");
                    if let Some(stream) = &stream_or_none {
                        stream.shutdown(Shutdown::Both);
                    }
                    connected = false;
                    setup(address);
                }
            } else if keys.intersects(KeyPad::KEY_X) && calibration_step == 0 {
                println!("Display calibration");
                println!(
                    "Press and hold the stylus on the top left corner of the screen, then press A."
                );
                calibration_step = 1;
            } else if keys.intersects(KeyPad::KEY_A) {
                if calibration_step == 1 {
                    println!(
                        "\u{001b}[32mTop left: ({}, {})\u{001b}[0m",
                        touch.0, touch.1
                    );
                    println!("Now press and hold the stylus on the top right corner of the screen, then press A.");
                    calibration_step = 2;
                } else if calibration_step == 2 {
                    println!(
                        "\u{001b}[32mTop right: ({}, {})\u{001b}[0m",
                        touch.0, touch.1
                    );
                    println!("Now press and hold the stylus on the bottom left corner of the screen, then press A.");
                    calibration_step = 3;
                } else if calibration_step == 3 {
                    println!(
                        "\u{001b}[32mBottom left: ({}, {})\u{001b}[0m",
                        touch.0, touch.1
                    );
                    println!("Now press and hold the stylus on the bottom right corner of the screen, then press A.");
                    calibration_step = 4;
                } else if calibration_step == 4 {
                    println!(
                        "\u{001b}[32mBottom right: ({}, {})\u{001b}[0m",
                        touch.0, touch.1
                    );
                    println!("Calibration completed!");
                    calibration_step = 0;
                }
            }
        }

        last_keys = keys;

        // Flush and swap framebuffers
        gfx.flush_buffers();
        gfx.swap_buffers();
        gfx.wait_for_vblank();
    }

    fn setup(address: Ipv4Addr) {
        println!("ctr-tablet-rs v1.0.0 by Lena");
        println!("https://github.com/adryzz/ctr-tablet-rs");
        println!("IP: {address}:5000");
        println!("Connect a PC to this console...");
        println!("Press START to exit or X to calibrate the display.");

        println!("\u{001b}[46;1m                \u{001b}[0m");
        println!("\u{001b}[45;1m                \u{001b}[0m");
        println!("\u{001b}[47m                \u{001b}[0m");
        println!("\u{001b}[45;1m                \u{001b}[0m");
        println!("\u{001b}[46;1m                \u{001b}[0m");
    }
}
