use std::env;

fn debug_enabled() -> bool {
    match env::var("DEBUG") {
        Ok(_) => true,
        Err(_) => false
    }
}

pub fn log(msg: &str) {
    if debug_enabled() {
        println!("{}",msg);
    }
}