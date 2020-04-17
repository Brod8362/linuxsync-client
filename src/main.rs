use std::net::{TcpStream, Shutdown};
use std::io::{Read, Write};
use std::{time};
use std::thread::sleep;
use std::str;
use std::collections::HashMap;
use serde_json::Value;
use crate::json_parser::get_devices;
use std::borrow::Borrow;
use std::time::Duration;

mod notifications;
mod debug;
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
    notifications::send_notification(elements.get(&1).unwrap_or(&""),
                                     elements.get(&2).unwrap_or(&""),
                                     elements.get(&3).unwrap_or(&""));
}

fn client_handler(mut stream: TcpStream) {
    let mut data = [0 as u8; 1024];
    while match stream.read(&mut data) {
        Ok(size) => {
            let mut cont = true;
            let res = stream.set_read_timeout(Option::from(Duration::from_millis(20000)));
            if res.is_err() {
                panic!("failed to set read timeout");
            }
            if size != 0 && data[0] == 0x3C {
                data_handler(data.as_ref(), size);
                debug::log(format!("notification of size {}", size).as_str());
            }
            if data[0]==0x7F && data[size]==0x7F {
                debug::log("shutdown signal received");
                let res = stream.shutdown(Shutdown::Both);
                if res.is_err() {
                }
                cont = false;
            }
            if data[0]==0x1A {
                let res = stream.write(&[0x1A as u8]);
                if res.is_err() {
                    //close the socket and start searching
                    cont = false;
                }
            }
            cont
        }
        Err(e) => {
            debug::log(format!("error reading stream data {:?}", e).as_str());
            let r = stream.shutdown(Shutdown::Both);
            if r.is_err() {
                //dont care
            }
            false
        }
    } {}
}

fn main() {
    let config_json: Value = json_parser::read_config_file();
    let devices = get_devices(config_json.borrow());
    debug::log(format!("found {} devices in config", devices.len()).as_ref());
    notifications::start_notification();
    debug::log("starting");
    loop {
        for device in &devices {
            debug::log(format!("trying to connect to {}", device.ip.as_str()).as_str());
            match TcpStream::connect(device.ip.as_str()) {
                Ok(stream) => {
                    debug::log(format!("connected to {}", device.ip.as_str()).as_str());
                    let addr = stream.peer_addr().unwrap();
                    notifications::connection_established(device.name.as_str(), addr);
                    client_handler(stream); //this returns when the connection is lost
                    notifications::connection_lost(device.name.as_str(), addr);
                    debug::log(format!("connection to {} lost", device.ip.as_str()).as_str());
                }
                Err(_) => {
                    //device unavailable
                }
            }
        }
        debug::log(format!("tried {} devices, none found. sleeping", devices.len()).as_ref());
        sleep(time::Duration::from_millis(10000));
    }
}
