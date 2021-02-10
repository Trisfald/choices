# `choices`

[![works badge](https://cdn.jsdelivr.net/gh/nikku/works-on-my-machine@v0.2.0/badge.svg)](https://github.com/nikku/works-on-my-machine)

Do you like `structops` and `clap`? 
Do you write `microservices`?
Continue reading!

`choices` is a library that lets you expose your application's configuration 
over HTTP with a simple struct!

## Look, it's easy

Given the following code:

```rust
use choices::Choices;
use lazy_static::lazy_static;
use std::sync::{Arc, Mutex};

#[derive(Choices)]
struct Config {
    debug: bool,
    id: Option<i32>,
    log_file: String,
}

lazy_static! {
    static ref CONFIG: Arc<Mutex<Config>> = {
        Arc::new(Mutex::new(Config {
            debug: false,
            id: Some(3),
            log_file: "log.txt".to_string()
        }))
    };
}

#[tokio::main]
async fn main() {
    CONFIG.run((std::net::Ipv4Addr::LOCALHOST, 8081)).await;
}
```

You can see all configuration fields at `localhost:8081/config` 
and the individual fields' values at `localhost:8081/config/<field name>`.\
A field's value can be changed with a `PUT`, for instance 
`curl -X PUT localhost:8081/config/debug -d "true"`.

More examples in [examples](/examples).

Also check out the [documentation](/documentation.md).

## Features

- [x] show all configuration fields
- [x] GET configuration field
- [x] PUT configuration field
- [x] user defined types
- [ ] JSON support
- [ ] custom validators
- [ ] on change callbacks

## Thanks

Special thanks to the authors of `structops`. It served as an inspiration to learn procedural macros.
