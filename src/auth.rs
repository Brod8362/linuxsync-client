use crate::paths;
use std::fs;

pub fn read_pubkey() -> String {
    let key: String = fs::read_to_string(paths::pubkey_file_path()).unwrap_or("".parse().unwrap());
    if key.eq("") {
       gen_keys();
        return read_pubkey();
    }
    key
}

pub fn read_private_key() -> String {
    let key: String = fs::read_to_string(paths::private_key_file_path()).unwrap_or("".parse().unwrap());
    if key.eq("") {
        gen_keys();
        return read_private_key();
    }
    key
}

fn gen_keys() {

}