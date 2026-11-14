use crate::{ResendServiceData, TypicalEmail};
use resend_rs::{Resend, types::{CreateEmailBaseOptions, CreateEmailResponse}};
use secrecy::ExposeSecret;

#[tracing::instrument]
pub async fn resend_send_email(
    data: ResendServiceData,
    email_data: TypicalEmail,
) -> anyhow::Result<()> {
    tracing::debug!("Sending via resend...");
    let ResendServiceData {
        sender_qualified_domain_name,
        api_key,
    } = data;
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
    let email_response = resend.emails.send(email).await.map_err(|e|anyhow::anyhow!("{e:#?}"));
    Ok(())

}
