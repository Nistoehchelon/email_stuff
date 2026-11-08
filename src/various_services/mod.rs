pub mod aha_send;
pub use aha_send::aha_send_send_email;
pub mod resend;
pub use resend::resend_send_email;
pub mod lettermint;
pub use lettermint::lettermint_send_email;

fn get_reqwest_client() -> reqwest::Client {
    reqwest::Client::new()
}