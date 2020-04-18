use std::path::PathBuf;
use std::ffi::OsString;

pub fn config_file_folder_pathbuf() -> PathBuf {
    let mut pathbuf:PathBuf = dirs::config_dir().unwrap();
    pathbuf.push("linuxsync");
    return pathbuf
}

pub fn config_file_folder_path() -> OsString {
    config_file_folder_pathbuf().into_os_string()
}

pub fn config_file_path() -> OsString {
    let mut pb: PathBuf = config_file_folder_pathbuf();
    pb.push("config.json");
    pb.into_os_string()
}

pub fn pubkey_file_path() -> OsString {
    let mut pb: PathBuf = config_file_folder_pathbuf();
    pb.push("key.pub");
    pb.into_os_string()
}

pub fn private_key_file_path() -> OsString {
    let mut pb: PathBuf = config_file_folder_pathbuf();
    pb.push("key");
    pb.into_os_string()
}