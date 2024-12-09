use anyhow::{Context, Result};
use chrono::Local;
use colored::*;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiResponse {
    status: Option<String>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Node {
    pub _id: String,
    #[serde(rename = "pubKey")]
    pub pub_key: String,
    #[serde(rename = "userId")]
    pub user_id: String,
    pub __v: u32,
    #[serde(rename = "createdAt")]
    pub created_at: String,
    #[serde(rename = "hardwareId")]
    pub hardware_id: String,
    #[serde(rename = "hardwareInfo")]
    pub hardware_info: Option<serde_json::Value>,
    #[serde(rename = "ipAddress")]
    pub ip_address: Option<String>,
    #[serde(rename = "updatedAt")]
    pub updated_at: String,
    pub sessions: Vec<Session>,
    #[serde(rename = "totalReward")]
    pub total_reward: u32,
    #[serde(rename = "todayReward")]
    pub today_reward: u32,
    #[serde(rename = "isConnected")]
    pub is_connected: bool,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Session {
    pub _id: String,
    #[serde(rename = "nodeId")]
    pub pub_key: String,
    #[serde(rename = "startAt")]
    pub start_at: String,
    pub pings: Vec<String>,
    pub __v: u32,
    #[serde(rename = "lastPingAt")]
    pub last_ping_at: Option<String>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Ping {
    pub timestamp: String,
    #[serde(rename = "nodeId")]
    pub node_id: String,
    #[serde(rename = "isB7SConnected")]
    pub is_b7s_connected: bool,
    pub _id: String,
    pub __v: u32,
}

#[derive(Deserialize, Debug, Clone)]
pub struct PingResponse {
    #[serde(rename = "activeSession")]
    pub active_session: Session,
    pub ping: Ping,
}

#[derive(Clone)]
pub struct Bless {
    access_token: String,
    client: Client,
}

impl Bless {
    const API_BASE_URL: &'static str = "https://gateway-run.bls.dev/api/v1";
    const X_EXTENSION_VERSION: &'static str = "0.1.5";

    pub fn new(client: Client, access_token: String) -> Self {
        Bless {
            access_token,
            client,
        }
    }

    pub async fn nodes(&self) -> Result<Vec<Node>> {
        let url = format!("{}/nodes", Self::API_BASE_URL);

        let response = self
            .client
            .get(&url)
            .header("Authorization", format!("Bearer {}", self.access_token))
            .send()
            .await
            .context("Failed to send request to fetch nodes")?;
        if !response.status().is_success() {
            return Err(anyhow::anyhow!(
                "Failed to fetch nodes: HTTP {}",
                response.status()
            ));
        }
        let nodes: Vec<Node> = response
            .json()
            .await
            .context("Failed to parse nodes data")?;
        Ok(nodes)
    }

    pub async fn register(
        &self,
        pub_key: &str,
        hardware_id: &str,
        addr: Option<&str>,
    ) -> Result<ApiResponse> {
        let url = format!("{}/nodes/{}", Self::API_BASE_URL, pub_key);
        println!(
            "[{}] Registering node PubKey: {:?}, HardwareID: {}",
            Local::now(),
            pub_key,
            hardware_id
        );

        let mut body = HashMap::new();
        if let Some(ip) = addr {
            body.insert("ipAddress", ip);
        }
        body.insert("hardwareId", hardware_id);

        let response = self
            .client
            .post(&url)
            .header("Authorization", format!("Bearer {}", self.access_token))
            .header("X-Extension-Version", Self::X_EXTENSION_VERSION)
            .json(&body)
            .send()
            .await
            .context("Failed to send registration request")?;

        if !response.status().is_success() {
            return Err(anyhow::anyhow!(
                "Failed to register node {}: HTTP {}",
                pub_key,
                response.status()
            ));
        }

        let response_text = response
            .text()
            .await
            .context("Failed to read response text")?;

        if cfg!(debug_assertions) {
            println!("[{}] Raw response body: {}", Local::now(), response_text);
        }

        let data = serde_json::from_str(&response_text)
            .context("Failed to parse registration response")?;

        println!("[{}] Registration: {:?}", Local::now(), pub_key);

        Ok(data)
    }

    pub async fn init_session(&self, pub_key: &str) -> Result<ApiResponse> {
        let url = format!("{}/nodes/{}/start-session", Self::API_BASE_URL, pub_key);
        println!(
            "[{}] Starting session for node {}, it might take a while...",
            Local::now(),
            pub_key
        );

        let response = self
            .client
            .post(&url)
            .header("Authorization", format!("Bearer {}", self.access_token))
            .header("X-Extension-Version", Self::X_EXTENSION_VERSION)
            .send()
            .await
            .context("Failed to start session")?;

        if !response.status().is_success() {
            return Err(anyhow::anyhow!(
                "Failed to start session for node {}: HTTP {}",
                pub_key,
                response.status()
            ));
        }

        let response_text = response
            .text()
            .await
            .context("Failed to read response text")?;

        if cfg!(debug_assertions) {
            println!("[{}] Raw response body: {}", Local::now(), response_text);
        }

        let data: ApiResponse =
            serde_json::from_str(&response_text).context("Failed to parse ping response")?;

        println!("[{}] Started session: {:?}", Local::now(), pub_key);

        Ok(data)
    }

    pub async fn stop_session(&self, pub_key: &str) -> Result<ApiResponse> {
        let url = format!("{}/nodes/{}/stop-session", Self::API_BASE_URL, pub_key);
        println!(
            "[{}] Stoping session for node {}, it might take a while...",
            Local::now(),
            pub_key
        );

        let response = self
            .client
            .post(&url)
            .header("Authorization", format!("Bearer {}", self.access_token))
            .header("X-Extension-Version", Self::X_EXTENSION_VERSION)
            .send()
            .await
            .context("Failed to stop session")?;

        if !response.status().is_success() {
            return Err(anyhow::anyhow!(
                "Failed to stop session for node {}: HTTP {}",
                pub_key,
                response.status()
            ));
        }

        let response_text = response
            .text()
            .await
            .context("Failed to read response text")?;

        if cfg!(debug_assertions) {
            println!("[{}] Raw response body: {}", Local::now(), response_text);
        }

        let data: ApiResponse =
            serde_json::from_str(&response_text).context("Failed to parse ping response")?;

        println!("[{}] Stoped session: {:?}", Local::now(), pub_key);

        Ok(data)
    }

    pub async fn ping(&self, pub_key: &str, addr: Option<&str>) -> Result<PingResponse> {
        let ping_url = format!("{}/nodes/{}/ping", Self::API_BASE_URL, pub_key);

        println!(
            "[{}] Pinging node {} with IP: {:?}",
            Local::now(),
            pub_key,
            addr.unwrap_or("null")
        );

        let mut body = HashMap::new();

        body.insert("isB7SConnected", true);

        let response = self
            .client
            .post(&ping_url)
            .header("Content-Type", "application/json")
            .header("Authorization", format!("Bearer {}", self.access_token))
            .header("X-Extension-Version", Self::X_EXTENSION_VERSION)
            .json(&body)
            .send()
            .await
            .context("Failed to send ping request")?;
            
        if !response.status().is_success() {
            return Err(anyhow::anyhow!(
                "Ping failed for node {}: HTTP {}",
                pub_key,
                response.status()
            ));
        }

        // Read and debug the response text
        let response_text = response
            .text()
            .await
            .context("Failed to read response text")?;

        if cfg!(debug_assertions) {
            println!("[{}] Raw response body: {}", Local::now(), response_text);
        }

        // Deserialize the JSON
        let data: PingResponse =
            serde_json::from_str(&response_text).context("Failed to parse ping response")?;

        let status_colored = if data.ping.is_b7s_connected {
            "OK".green()
        } else {
            "FAIL".red()
        };

        println!(
            "[{}] Ping response status: {}, PubKey: {}, IP: {:?}",
            Local::now(),
            status_colored,
            pub_key.cyan(),
            addr.unwrap_or("null")
        );

        Ok(data)
    }
}
