//! Validate a configuration field's value with user defined functions.

use choices::{Choices, ChoicesError, ChoicesResult};
use lazy_static::lazy_static;
use std::sync::{Arc, Mutex};

#[derive(Choices)]
struct Config {
    // Use a free function.
    #[choices(validator = check_port)]
    port: u16,
    // Use a boxed function stored in this struct.
    #[choices(validator = (self.file_validator))]
    file: String,
    #[choices(skip)]
    file_validator: Box<dyn Fn(&String) -> ChoicesResult<()> + Send + Sync>,
}

lazy_static! {
    static ref CONFIG: Arc<Mutex<Config>> = Arc::new(Mutex::new(Config {
        port: 10000,
        file: String::from("tmp"),
        file_validator: Box::new(|_| { Ok(()) }),
    }));
}

fn check_port(port: &u16) -> ChoicesResult<()> {
    if *port > 1000 {
        Ok(())
    } else {
        Err(ChoicesError::ValidationError(format!(
            "value must be greater than 1000"
        )))
    }
}

#[tokio::main]
async fn main() {
    // Set a config field directly. This won't perform the validation.
    CONFIG.lock().unwrap().port = 100;

    // Set a config field through its setter. This will validate the new value.
    CONFIG.lock().unwrap().set_port(4200u16).unwrap();

    // Set a custom validator for file.
    CONFIG.lock().unwrap().file_validator = Box::new(|v| {
        if !v.is_empty() {
            Ok(())
        } else {
            Err(ChoicesError::ValidationError(format!("invalid value")))
        }
    });
    CONFIG
        .lock()
        .unwrap()
        .set_file("file_2".to_string())
        .unwrap();

    CONFIG.run((std::net::Ipv4Addr::LOCALHOST, 8081)).await;

    // To change port: curl -X PUT localhost:8081/config/port -d "5"
    // To change file: curl -X PUT localhost:8081/config/file -d "file.txt"
}
