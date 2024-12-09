use anyhow::{anyhow, Result};
// use chrono::Local;
use reqwest::{Client, Proxy, Url};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Config {
    pub token: String,
    pub proxy: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct IpResponse {
    ip: String,
}

pub async fn what_my_addr(client: &Client) -> Result<String> {
    let response = client
        .get("https://tight-block-2413.txlabs.workers.dev")
        .send()
        .await?;
    let data: IpResponse = response.json().await?;
    // println!("[{}] IP fetch response: {:?}", Local::now(), data);
    Ok(data.ip)
}

pub fn create_client(user: &Config) -> Result<Client> {
    let client = Client::builder();
    let client = if let Some(url) = &user.proxy {
        let proxy_url = Url::parse(url)?;
        let host = proxy_url.host_str().unwrap_or("127.0.0.1");
        let protocol = proxy_url.scheme();
        let port = proxy_url.port().unwrap_or(80);
        let username = proxy_url.username();
        let password = proxy_url.password().unwrap_or("");

        let proxy = Proxy::https(format!("{}://{}:{}", protocol, host, port))?;

        let proxy = if username.is_empty() {
            proxy
        } else {
            proxy.basic_auth(username, password)
        };

       client.proxy(proxy)
    } else {
        client
    };

    client
        .build()
        .map_err(|e| anyhow!("Failed to build client: {}", e))
}
