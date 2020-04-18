use crate::{paths, debug};
use std::fs;
use openssl::rsa::Rsa;
use openssl::pkey::{PKey, Public, Private};
use std::fs::File;
use std::io::Write;

pub fn read_pubkey() -> PKey<Public> {
    let key_r= fs::read(paths::pubkey_file_path());
    if key_r.is_err() {
       gen_keys();
        return read_pubkey();
    }
    PKey::public_key_from_pem(key_r.unwrap().as_slice()).unwrap()
}

pub fn read_private_key() -> PKey<Private> {
    let key_r= fs::read(paths::private_key_file_path());
    if key_r.is_err() {
        gen_keys();
        return read_private_key();
    }
    PKey::private_key_from_der(key_r.unwrap().as_slice()).unwrap()
}

fn gen_keys() {
    debug::log("generating new authentication keys");
    let keypair = Rsa::generate(2048).unwrap();
    let pkey = PKey::from_rsa(keypair).unwrap();

    let pub_key: Vec<u8> = pkey.public_key_to_pem().unwrap();
    let priv_key: Vec<u8> = pkey.private_key_to_der().unwrap();

    let pfile = File::create(paths::pubkey_file_path());
    match pfile {
        Ok(mut file) => {
            let res = file.write_all(pub_key.as_slice());
            if res.is_err() {
                debug::log_err(format!("error writing pubkey {}", res.unwrap_err()).as_str());
            }
        }
        Err(e) => {
            debug::log_err(format!("error writing pubkey {}", e).as_str());
        }
    }

    let pfile = File::create(paths::private_key_file_path());
    match pfile {
        Ok(mut file) => {
            let res = file.write_all(priv_key.as_slice());
            if res.is_err() {
                debug::log_err(format!("error writing private key {}", res.unwrap_err()).as_str());
            }
        }
        Err(e) => {
            debug::log_err(format!("error writing private key {}", e).as_str());
        }

    }
}