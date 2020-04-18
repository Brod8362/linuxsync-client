use notify_rust::Notification;
use std::net::SocketAddr;
use std::collections::HashMap;
use crate::debug;
use crate::protos::packet::NotificationData_Action;

type NotificationCallback = fn(&str);

pub fn start_notification() {
    send_meta_notification("Searching for devices");
}

pub fn connection_established(name: &str, addr: SocketAddr) {
    send_meta_notification(format!("Connected to {} ({})", name, addr).as_str());
}

pub fn connection_lost(name: &str, addr: SocketAddr) {
    send_meta_notification(format!("Connection to {} ({}) lost.", name, addr).as_str());
}

pub fn connection_failed(name: &str, addr: SocketAddr) {
    send_meta_notification(format!("Failed to connect to {} ({})", name, addr).as_str());
}

pub fn connection_denied(name: &str, addr: SocketAddr) {
    send_meta_notification(format!("{} ({}) denied the connection.", name, addr).as_str());
}

pub fn send_meta_notification(text: &str) {
    send_notification("LinuxSync", text, "")
}

pub fn send_notification(title: &str, text: &str, appname: &str) {
    let result = Notification::new()
        .summary(title)
        .body(text)
        .subtitle(appname)
        .show();
    if result.is_err() {
        println!("some notification showing error");
    }
}

pub fn send_notification_proto(title: &str, body: &str, appname: &str, id: i32,
                               actions: &[NotificationData_Action], callback: NotificationCallback) {
    let mut notification = Notification::new();
    notification.summary(title);
    notification.body(body);
    notification.subtitle(appname);

    for action in actions.iter() {
        notification.action(format!("{}-{}", id, action.get_index()).as_str(), action.get_title());
    }
    show_notification_with_actions(notification, callback);
}

pub fn send_notification_maps(elements: HashMap<u8, &str>, actions: HashMap<&str, i8>,
                              callback: NotificationCallback) {
    debug::log("preparing to send notification");
    let title = elements.get(&1).unwrap_or(&"<no title>");
    let body = elements.get(&2).unwrap_or(&"<no body>");
    let appname = elements.get(&4).unwrap_or(&"<no appname>");
    let id = elements.get(&5).unwrap_or(&"0"); //ID should always be present

    let mut notification = Notification::new();
    notification.summary(title);
    notification.body(body);
    notification.subtitle(appname);

    for (k, v) in actions.iter() {
        notification.action(format!("{}-{}", id, v).as_str(), k);
    }
    show_notification_with_actions(notification, callback);
}

fn show_notification_with_actions(notification: Notification, _callback: NotificationCallback) {
    match notification.show() {
        Ok(_nf) => {
            // nf.wait_for_action(|id| match id {
            //     "__closed" => {}
            //     _ => {
            //         callback(id);
            //     }
            // });
            /* blocking call, needs to be disabled */
        }
        Err(e) => {
            eprintln!("{:?}", e);
            panic!("failed to show notification");
        }
    }
}
