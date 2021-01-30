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

#[derive(Choices)]
struct Config {
    debug: bool,
    id: Option<i32>,
    log_file: String,
}

lazy_static! {
    static ref CONFIG: Config = {
        Config {
            debug: false,
            id: Some(3),
            log_file: "log.txt".to_string()
        }
    };
}

#[tokio::main]
async fn main() {
    CONFIG.run(([127, 0, 0, 1], 8081)).await;
}
```

You can see all configuration fields at `localhost:8081/config` 
and the individual fields' values at `localhost:8081/config/<field name>`.

More examples in [examples](/examples).

## Thanks

Special thanks to the authors of `structops`. It served as an inspiration to learn procedural macros.
