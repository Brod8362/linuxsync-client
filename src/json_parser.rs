use std::fs;
use std::fs::File;
use serde_json::Value;
use std::io::Write;

use crate::paths;

#[derive(Debug)]
pub struct Device {
    pub name: String,
    pub ip: String,
    pub mac: String,
}


pub fn read_config_file() -> Value {
    let data_r = fs::read_to_string(paths::config_file_path());
    if data_r.is_err() {
        create_config_file();
        return read_config_file();
    }
    let data = data_r.unwrap();
    let parsed = serde_json::from_str(data.as_str());
    if parsed.is_err() {
        println!("config file error, terminating");
        println!("{:?}", parsed.err());
        std::process::exit(1);
    }
    return parsed.unwrap();
}

pub fn get_devices(json: &Value) -> Vec<Device> {
    let raw_vec = json["devices"].as_array().unwrap();
    let a: Vec<Device> = raw_vec.into_iter().map(value_to_device).collect();
    a
}

fn value_to_device(v: &Value) -> Device {
    Device {
        name: v["name"].as_str().unwrap().parse().unwrap(),
        ip: v["ip"].as_str().unwrap().parse().unwrap(),
        mac: v["mac"].as_str().unwrap().parse().unwrap(),
    }
}

fn create_config_file() -> bool {
    let res1 = fs::create_dir_all(paths::config_file_folder_path());
    let res2 = File::create(paths::config_file_path());
    if res2.is_ok() {
        let data = include_bytes!("res/default.json");
        let res2 = res2.unwrap().write_all(data);
        if res2.is_err() {
            println!("failed to write new config file");
            println!("{:?}", res2.err());
        }
        return true;
    }
    if res1.is_err() || res2.is_err() {
        panic!("failed to create configuration file");
    }
    false
}

