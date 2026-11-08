use reqwest::Client;
use std::time::Duration;

const REQUEST_TIMEOUT: Duration = Duration::from_secs(30);
const CONNECT_TIMEOUT: Duration = Duration::from_secs(10);
const USER_AGENT: &str = concat!("wenmoon/", env!("CARGO_PKG_VERSION"));

pub fn client() -> Client {
    Client::builder()
        .timeout(REQUEST_TIMEOUT)
        .connect_timeout(CONNECT_TIMEOUT)
        .user_agent(USER_AGENT)
        .build()
        .expect("failed to build the HTTP client")
}
