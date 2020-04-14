use std::net::{TcpStream, Shutdown, SocketAddr};
use std::io::Read;
use std::time;
use notify_rust::Notification;
use std::thread::sleep;
use std::str;
use std::collections::HashMap;

fn start_notification() {
    let result = Notification::new()
        .summary("LinuxSync")
        .body("Searching for phone...")
        .show();
}

fn connection_established(addr: SocketAddr) {
    let result = Notification::new()
        .summary("LinuxSync")
        .body(format!("Connected to {}", addr).as_str())
        .show();
}

fn connection_lost(addr: SocketAddr) {
    let result = Notification::new()
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
    println!("{:?}", data);
    let mut elements: HashMap<u8, &str> = HashMap::new();
    let segments = data[1] as usize;
    let mut read = 2;
    for x in 0..segments {
        let segment_type = data[read];
        read+=1;
        let segment_length = data[read] as usize;
        read+=1;
        let data_r = std::str::from_utf8(&data[read..read+segment_length]);
        read+=segment_length;
        if data_r.is_err() {
            //do something bad
            continue;
        }
        let data_f = data_r.unwrap();
        elements.insert(segment_type, data_f);
    }
    assert_eq!(data[read], 127); //if this assertion fails, invalid packet data was sent
    notification(elements.get(&1).unwrap(),
                 elements.get(&2).unwrap());
}

fn client_handler(mut stream: TcpStream) {
    let mut data = [0 as u8; 1024];
    while match stream.read(&mut data) {
        Ok(size) => {
            println!("data read");
            if size != 0 && data[0] == 0x3C {
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
