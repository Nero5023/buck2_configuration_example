#[cfg(feature = "logging")]
fn log_message(msg: &str) {
    println!("[my_feature_lib LOG]: {}", msg);
}

#[cfg(not(feature = "logging"))]
fn log_message(msg: &str) {
    // do nothing
}


#[cfg(feature = "timestamp")]
pub fn print_timestamp() {
    use std::time::{SystemTime, UNIX_EPOCH};
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();
    println!("[{}]", timestamp);
}

#[cfg(not(feature = "timestamp"))]
pub fn print_timestamp() {
    // do nothing
}

pub fn print_hello(name: &str) {
    print_timestamp();
    log_message(&format!("Accepting name: {}", name));
    println!("Hello, {}!", name);
}