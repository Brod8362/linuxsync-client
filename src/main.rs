use std::net::{TcpStream, Shutdown};
use std::io::{Read, Write};
use std::time;
use std::thread::sleep;
use std::str;
use serde_json::Value;
use crate::json_parser::{get_devices, Device};
use std::borrow::Borrow;
use std::time::Duration;
use crate::protos::auth::ClientDetails;
use protobuf::Message;
use openssl::pkey::Private;
use openssl::rsa::Rsa;

mod notifications;
mod debug;
mod json_parser;
mod protos;
mod paths;
mod auth;
mod data_handlers;

fn action_event(id: &str) {
    println!("{}", id);
}

fn client_handler(mut stream: &TcpStream, private_key: &Rsa<Private>) {
    let mut data = [0 as u8; 4096];
    while match stream.read(&mut data) {
        Ok(size) => {
            let mut cont = true;
            let res = stream.set_read_timeout(Option::from(Duration::from_millis(20000)));
            if res.is_err() {
                panic!("failed to set read timeout");
            }
            if size != 0 && data[0] == 0x3C {
                data_handlers::data_handler_legacy(data.as_ref(), size);
                debug::log(format!("notification of size {}", size).as_str());
            }
            if size != 0 && data[0] == 0x3D {
                data_handlers::data_handler_protobuf(&data[1..size]);
            }
            if size != 0 && data[0] == 0x3E {
                data_handlers::data_handler_rsa(&data[1..size], private_key);
            }
            if data[0] == 0x7F && data[size] == 0x7F {
                debug::log("shutdown signal received");
                let res = stream.shutdown(Shutdown::Both);
                if res.is_err() {}
                cont = false;
            }
            if data[0] == 0x1A {
                let res = stream.write(&[0x1A as u8]);
                if res.is_err() {
                    //close the socket and start searching
                    cont = false;
                }
            }
            cont
        }
        Err(e) => {
            debug::log_err(format!("error reading stream data {:?}", e).as_str());
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

    let hostname_option = config_json["hostname"].as_str();
    if hostname_option.is_none() {
        panic!("failed to read hostname from config file");
    }
    let hostname = hostname_option.unwrap();

    let pubkey = auth::read_pubkey();
    let private_key = auth::read_private_key();

    let mut device_data = ClientDetails::new();
    device_data.set_hostname(hostname.parse().unwrap());
    let pem: Vec<u8> = pubkey.public_key_to_der().unwrap();
    device_data.set_pubkey(pem);

    let devices = get_devices(config_json.borrow());

    debug::log(format!("hostname: {}\nfound {} devices in config", hostname, devices.len()).as_ref());
    notifications::start_notification();
    debug::log("starting");
    loop {
        for device in &devices {
            debug::log(format!("connecting: {}", device.ip.as_str()).as_str());
            match TcpStream::connect(device.ip.as_str()) {
                Ok(stream) => {
                    if authenticate(&mut device_data, device, &stream) {
                        debug::log(format!("connected: {}", device.ip.as_str()).as_str());
                        notifications::connection_established(device.name.as_str());
                        client_handler(&stream, &private_key); //this returns when the connection is lost
                        notifications::connection_lost(device.name.as_str());
                        debug::log(format!("disconnected: {}", device.ip.as_str()).as_str());
                    } else {
                        debug::log(format!("denied: {}", device.ip.as_str()).as_str());
                        notifications::connection_denied(device.name.as_str());
                    }
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

fn authenticate(device_data: &mut ClientDetails, device: &Device, mut stream: &TcpStream) -> bool {
    debug::log(format!("authenticating: {}", device.ip.as_str()).as_str());
    let res = stream.write(&device_data.write_to_bytes().unwrap());
    if res.is_err() {
        debug::log_err(format!("connected failed at hostname: {:?}", res.unwrap_err()).as_str());
        notifications::connection_failed(device.name.as_str());
        return false;
    }
    let mut data = [0 as u8; 32];
    let mut size = 0;
    match stream.set_read_timeout(Option::from(Duration::from_millis(20000))) {
        Ok(()) => debug::log("set initial read timeout"),
        Err(e) => {
            debug::log_err(format!("failed to set initial read timeout: {:?}", e).as_str());
        }
    }
    while size == 0 {
        let res = stream.read(&mut data);
        match res {
            Ok(s) => size = s,
            Err(e) => {
                debug::log_err(format!("connected failed at read: {:?}", e).as_str());
                return false;
            }
        }
    }
    return data[0] == 0xAC
}
