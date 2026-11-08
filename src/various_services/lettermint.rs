use crate::{LetterMintProjectToken, TypicalEmail};
use lettermint::{Lettermint, SendOptions};
use secrecy::ExposeSecret;

pub async fn lettermint_send_email(
    lettermint_project_token: LetterMintProjectToken,
    email_data: TypicalEmail,
) -> anyhow::Result<()> {
    let lm = Lettermint::builder()
        .sending_token(lettermint_project_token.0.expose_secret())
        .build()?;
    let sender_name = email_data.from_email_address;
    let sender_email = email_data.from_name;
    let receiver_email = email_data.to_email_address;
    let resp = lm
        .emails()
        .compose()
        .from(format!("{sender_name} <{sender_email}>"))
        .to(receiver_email)
        .subject(email_data.message_title)
        .html(email_data.message_content.clone())
        .text(email_data.message_content)
        .send(SendOptions::new())
        .await?;
    tracing::debug!("{resp:?}");

    Ok(())
}
