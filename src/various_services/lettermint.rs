use crate::{LettermintServiceData, TypicalEmail};
use lettermint::{Lettermint, SendOptions};
use secrecy::ExposeSecret;

#[tracing::instrument]
pub async fn lettermint_send_email(
    data: LettermintServiceData,
    email_data: TypicalEmail,
) -> anyhow::Result<()> {
    let LettermintServiceData {
        sender_qualified_domain_name,
        project_token,
    } = data;
    let TypicalEmail {
        from_email_username_part,
        from_name,
        to_email_address,
        to_name: _,
        message_title,
        message_content,
    } = email_data;
    let lm = Lettermint::builder()
        .sending_token(project_token.0.expose_secret())
        .build()?;
    let sender_name = from_name;
    let receiver_email = to_email_address;
    let resp = lm
        .emails()
        .compose()
        .from(format!(
            "{sender_name} <{from_email_username_part}@{sender_qualified_domain_name}>"
        ))
        .to(receiver_email)
        .subject(message_title)
        .html(message_content.clone())
        .text(message_content)
        .send(SendOptions::new())
        .await?;

    tracing::debug!("{resp:?}");
    Ok(())
}
