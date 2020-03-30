use std::net::{TcpListener, TcpStream, Shutdown, SocketAddr};
use std::io::Read;
use std::{thread, time};
use notify_rust::Notification;
use std::thread::sleep;

fn start_notification() {
    Notification::new()
        .summary("LinuxSync")
        .body("Searching for phone...")
        .show();
}

fn connection_established(addr: SocketAddr) {
    Notification::new()
        .summary("LinuxSync")
        .body(format!("Connected to {}", addr).as_str())
        .show();
}

fn connection_lost(addr: SocketAddr) {
    Notification::new()
        .summary("LinuxSync")
        .body(format!("Connection to {} lost.", addr).as_str())
        .show();
}

fn data_handler(mut data: [u8; 50], size: usize) {
    println!("data recv size {}", size);
}

fn client_handler(mut stream: TcpStream) {
    let mut data = [0 as u8; 50]; //50 byte buffer, will need to be larger
    while match stream.read(&mut data) {
        Ok(size) => {
            if size==0 {
                //error condition, i dont know how this happened
                return;
            } else {
                data_handler(data, size)
            }
            true
        }
        Err(e) => {
            println!("error reading stream data {}", e);
            stream.shutdown(Shutdown::Both).unwrap();
            false
        }
    } {}
}

fn main() {
    start_notification();
    while true {
        match TcpStream::connect("192.168.1.194:5000") {
            Ok(mut stream) => {
                let addr = stream.peer_addr().unwrap();
                connection_established(addr);
                client_handler(stream); //this returns when the connection is lost
                connection_lost(addr);
            }
            Err(_) => {
                sleep(time::Duration::from_millis(6000));
            }
        }
    }
}
