use std::net::{TcpStream, Shutdown};
use std::io::Read;
use std::time;
use std::thread::sleep;
use std::str;
use std::collections::HashMap;
use serde_json::Value;
use crate::json_parser::get_devices;
use std::borrow::Borrow;

mod notifications;
mod json_parser;

fn data_handler(data: &[u8], _size: usize) {
    assert_eq!(data[0], 0x3C);
    let mut elements: HashMap<u8, &str> = HashMap::new();
    let segments = data[1] as usize;
    let mut read = 2;
    for _ in 0..segments {
        let segment_type = data[read];
        read += 1;
        let segment_length = data[read] as usize;
        read += 1;
        let data_r = std::str::from_utf8(&data[read..read + segment_length]);
        read += segment_length;
        if data_r.is_err() {
            //do something bad
            continue;
        }
        let data_f = data_r.unwrap();
        elements.insert(segment_type, data_f);
    }
    assert_eq!(data[read], 127); //if this assertion fails, invalid packet data was sent
    notifications::send_notification(elements.get(&1).unwrap(),
                                     elements.get(&2).unwrap());
}

fn client_handler(mut stream: TcpStream) {
    let mut data = [0 as u8; 1024];
    let mut read = true;
    while read && match stream.read(&mut data) {
        Ok(size) => {
            if size != 0 && data[0] == 0x3C {
                data_handler(data.as_ref(), size);
            }
            if data[0]==0x7F && data[size]==0x7F {
                let res = stream.shutdown(Shutdown::Both);
                if res.is_err() {
                    //dont care
                }
                read = false;
            }
            true
        }
        Err(e) => {
            println!("error reading stream data {:?}", e);
            let r = stream.shutdown(Shutdown::Both);
            if r.is_err() {
                //dont care
            }
            read = false;
            false
        }
    } {}
}

fn main() {
    let config_json: Value = json_parser::read_config_file();
    let devices = get_devices(config_json.borrow());
    notifications::start_notification();
    loop {
        for device in &devices {
            match TcpStream::connect(device.ip.as_str()) {
                Ok(stream) => {
                    let addr = stream.peer_addr().unwrap();
                    notifications::connection_established(device.name.as_str(), addr);
                    client_handler(stream); //this returns when the connection is lost
                    notifications::connection_lost(device.name.as_str(), addr);
                }
                Err(_) => {
                    //device unavailable
                }
            }
        }
        sleep(time::Duration::from_millis(10000));
    }
}
