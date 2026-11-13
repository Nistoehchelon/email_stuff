use resend_rs::{Resend, types::CreateEmailBaseOptions};
use secrecy::ExposeSecret;
use crate::{ResendServiceData, TypicalEmail};

pub async fn resend_send_email(data: ResendServiceData, email_data: TypicalEmail) {
    let ResendServiceData { sender_qualified_domain_name, api_key } = data;
    let resend = {
        let raw_key: &str = api_key.0.expose_secret();
        Resend::new(raw_key)
    };
    let TypicalEmail {
        from_email_username_part,
        from_name,
        to_email_address,
        to_name: _,
        message_title,
        message_content,
    } = email_data;
    let from_email_address = format!("{from_email_username_part}@{sender_qualified_domain_name}");
    let from = format!("{from_name} <{from_email_address}>");
    let recipient_addr = format!("{to_email_address}");
    let recipient_addr_str = recipient_addr.as_str();
    let to = [recipient_addr_str];
    let email = CreateEmailBaseOptions::new(from, to, message_title).with_html(&message_content);
    let email_response = resend.emails.send(email).await;
    tracing::debug!("CreateEmailResponse: {email_response:?}");
}
