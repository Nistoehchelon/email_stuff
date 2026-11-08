use resend_rs::{Resend, types::CreateEmailBaseOptions};
use secrecy::{ExposeSecret, SecretString};


pub struct TypicalEmail {
    pub from_email_address: String,
    pub from_name: String,
    pub to_email_address: String,
    pub to_name: String,
    pub message_title: String,
    pub message_content: String,
}

pub async fn resend_send_email(
    resend_api_key: SecretString,
    email_data: TypicalEmail,
) {
    let resend = {
        let raw_key: &str = resend_api_key.expose_secret();
        Resend::new(raw_key)
    };
    let TypicalEmail {
        from_email_address,
        from_name,
        to_email_address,
        to_name: _,
        message_title,
        message_content,
    } = email_data;
    let from = format!("{from_name} <{from_email_address}>");
    let recipient_addr = format!("{to_email_address}");
    let recipient_addr_str = recipient_addr.as_str();
    let to = [recipient_addr_str];
    let email = CreateEmailBaseOptions::new(from, to, message_title).with_html(&message_content);
    let email_response = resend.emails.send(email).await;
    tracing::debug!("CreateEmailResponse: {email_response:?}");
}
