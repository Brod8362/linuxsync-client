use notify_rust::Notification;
use std::net::SocketAddr;

pub fn start_notification() {
    send_meta_notification("Searching for devices");
}

pub fn connection_established(name: &str, addr: SocketAddr) {
    send_meta_notification(format!("Connected to {} ({})", name, addr).as_str());
}

pub fn connection_lost(name: &str, addr: SocketAddr) {
    send_meta_notification(format!("Connection to {} ({}) lost.", name, addr).as_str());
}

pub fn send_meta_notification(text: &str) {
    send_notification("LinuxSync", text)
}

pub fn send_notification(title: &str, text: &str) {
    let result = Notification::new()
        .summary(title)
        .body(text)
        .show();
    if result.is_err() {
        println!("some notification showing error");
    }
}
