use choices::Choices;
use tokio::runtime::Runtime;

#[macro_use]
mod util;

#[tokio::test]
async fn get_list() {
    let port = file_line_port!();

    #[derive(Choices)]
    struct Config {
        debug: bool,
        retries: u8,
        delay: f64,
        score: Option<i32>,
    }

    let rt = Runtime::new().unwrap();
    rt.spawn(async move {
        Config {
            debug: true,
            retries: 3,
            delay: 0.1,
            score: Some(3),
        }
        .run(([127, 0, 0, 1], port))
        .await
    });

    let response =
        retry_await!(reqwest::get(&format!("http://127.0.0.1:{}/config", port))).unwrap();

    assert_eq!(response.status(), 200);
    let body = response.text().await.unwrap();
    assert_eq!(body, "Available configuration options:\n  - debug: bool\n  - retries: u8\n  - delay: f64\n  - score: Option<i32>\n");

    rt.shutdown_background();
}
