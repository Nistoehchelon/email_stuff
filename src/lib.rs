use std::net::IpAddr;

#[derive(serde::Deserialize, Debug)]
pub struct EmailSenderConfig {
    pub ip: IpAddr,
    pub port: u64,
    pub rust_log_directory_path: String,
    pub api_key: String,
}

#[derive(serde::Deserialize, Debug)]
pub enum ServiceType {
    Resend,
}

pub async fn send_email(title: &str, message: &str, recipient_email_address: &str) {}
