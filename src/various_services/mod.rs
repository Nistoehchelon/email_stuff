
pub mod aha_send;
pub use aha_send::ahasend_send_email;
pub mod resend;
pub use resend::resend_send_email;
pub mod lettermint;
pub use lettermint::lettermint_send_email;
use html2text::from_read;

fn get_reqwest_client() -> reqwest::Client {
    reqwest::Client::new()
}


fn html_to_text_content(html: &str) -> Result<String, html2text::Error> {
    from_read(html.as_bytes(), 80)
}