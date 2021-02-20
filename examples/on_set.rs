//! Invoke a function each time a new configuration value is set.

use choices::Choices;
use lazy_static::lazy_static;
use std::sync::{Arc, Mutex};

#[derive(Choices)]
struct Config {
    // Use a free function as callback.
    #[choices(on_set = print_port)]
    port: u16,
    // Use a boxed function stored in this struct.
    #[choices(on_set = (self.file_callback))]
    file: String,
    // You can even use a lambda.
    #[choices(on_set = (|v| println!("user: {}", v)))]
    user: String,
    #[choices(skip)]
    file_callback: Box<dyn Fn(&String) + Send + Sync>,
}

lazy_static! {
    static ref CONFIG: Arc<Mutex<Config>> = Arc::new(Mutex::new(Config {
        port: 10,
        file: String::from("tmp"),
        user: String::from("tom"),
        file_callback: Box::new(|_| {}),
    }));
}

fn print_port(port: &u16) {
    println!("new port: {}", port);
}

#[tokio::main]
async fn main() {
    // Set a config field directly. This won't invoke the callback.
    CONFIG.lock().unwrap().port = 100;

    // Set a config field through its setter. This will invoke the callback.
    CONFIG.lock().unwrap().set_port(42u16);

    // The default callback for file does nothing.
    CONFIG.lock().unwrap().set_file("file_1".to_string());
    // Set a callback that prints the new value.
    CONFIG.lock().unwrap().file_callback = Box::new(|v| println!("new file: {}", v));
    CONFIG.lock().unwrap().set_file("file_2".to_string());

    CONFIG.run((std::net::Ipv4Addr::LOCALHOST, 8081)).await;

    // To change port: curl -X PUT localhost:8081/config/port -d "5"
    // To change file: curl -X PUT localhost:8081/config/file -d "file.txt"
    // To change user: curl -X PUT localhost:8081/config/user -d "pablo"
}
