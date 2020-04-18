use std::convert::AsMut;
use openssl::pkey::Private;
use openssl::rsa::Padding;
use crate::{debug, notifications, action_event};
use crate::protos::packet::NotificationData;
use std::collections::HashMap;
use openssl::rsa::Rsa;

pub fn data_handler_rsa(data: &[u8], key: &Rsa<Private>) {
    let mut buf = [0 as u8; 1024];
    let res = key.private_decrypt(data, buf.as_mut(), Padding::PKCS1_OAEP);
    if res.is_err() {
        debug::log_err("failed to decrypt")
    } else {
        let size = res.unwrap();
        data_handler_protobuf(&buf[0..size]);
    }

}

pub fn data_handler_protobuf(data: &[u8]) {
    let proto: NotificationData = protobuf::parse_from_bytes::<NotificationData>(data).unwrap();
    notifications::send_notification_proto(proto.get_title(), proto.get_body(),
                                           proto.get_app_package(), proto.get_id(),
                                           proto.get_actions(), action_event);
}

pub fn data_handler_legacy(data: &[u8], _size: usize) {
    assert_eq!(data[0], 0x3C);
    let mut elements: HashMap<u8, &str> = HashMap::new();
    let mut actions: HashMap<&str, i8> = HashMap::new();
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

        if segment_type == 0x08 { //special case for action segments as they need to be handled differently
            let action_index = data[read] as i8;
            read += 1;
            actions.insert(data_f, action_index);
        } else {
            elements.insert(segment_type, data_f);
        }
    }
    assert_eq!(data[read], 127); //if this assertion fails, invalid packet data was sent
    notifications::send_notification_maps(elements, actions, action_event);
}