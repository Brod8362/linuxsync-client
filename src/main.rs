use std::net::{TcpStream, Shutdown, SocketAddr};
use std::io::Read;
use std::time;
use notify_rust::Notification;
use std::thread::sleep;
use std::str;

enum PacketSegmentType {
    Title = 0x01,
    Body = 0x02,
    Image = 0x03,
    End = 0x7F
}

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

fn notification(title: &str, text: &str) {
    Notification::new()
        .summary(title)
        .body(text)
        .show();
}

fn data_handler(mut data: &[u8], size: usize) {
    assert_eq!(data[0], 0x3C);
    let segments = data[1].to_usize();
    let mut read = 2;
    for x in 0..segments {
        let segment_type = data[read].to_usize();
        println!("Type is {}", segment_type);
        read+=1;
        let segment_length = data[read].to_usize();
        println!("Length is {}", segment_length);
        read+=1;
        let data_r = std::str::from_utf8(&data[read..segment_length]);
        read+=segment_length;
        if data_r.is_err() {
            //do something bad
            continue;
        }
        let data_f = data_r.unwrap();
        println!("data: {}", data_f);
    }
    assert_eq!(data[read+1], PacketSegmentType::End);
}

fn client_handler(mut stream: TcpStream) {
    let mut data = [0 as u8; 1024];
    while match stream.read(&mut data) {
        Ok(size) => {
            if size != 0 {
                data_handler(data.as_ref(), size);
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
    loop {
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
