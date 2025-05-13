use std::env;
use std::net::UdpSocket;
use std::str;
use std::time::Duration;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <ip>", args[0]);
        std::process::exit(1);
    }

    let payload = b"KNOCK KNOCK";
    let host = args[1].clone();
    let port = ":60400";
    let destination = format!("{}{}", host, port);
    println!("Sending payload to {}", destination);

    let socket = UdpSocket::bind("0.0.0.0:0").expect("Could not bind to socket");
    socket.send_to(payload, destination).expect("Could not send data");
    show_reply(socket);
}

fn show_reply(socket: UdpSocket){
    let mut buf = [0; 1024];
    socket.set_read_timeout(Some(Duration::new(5, 0))).expect("Could not set read timeout");
    match socket.recv_from(&mut buf) {
        Ok((size, src)) => {
            match str::from_utf8(&buf) {
                Ok(data) => {
                    println!("Received {} bytes from {}: {}", size, src, data);
                }
                Err(e) => {
                    eprintln!("Error converting data to string: {}", e);
                }
            }
        }
        Err(e) => {
            eprintln!("Error receiving data: {}", e);
        }
    }
}
