use secrecy::ExposeSecret;

use crate::{
    AhaSendAPIKey, AhaSendAccountId, AhasendServiceData, TypicalEmail,
    various_services::{aha_send, get_reqwest_client},
};

#[derive(Debug, serde::Serialize)]
struct Entity {
    email: String,
    name: String,
}

#[derive(Debug, serde::Serialize)]
pub struct EmailPayloadShape {
    sandbox: bool,
    from: Entity,
    recipients: Vec<Entity>,
    subject: String,
    text_content: String,
    html_content: String,
}
#[derive(Debug)]
pub struct AhaSend {
    api_key: AhaSendAPIKey,
    account_id: AhaSendAccountId,
}
impl AhaSend {
    pub fn new(api_key: AhaSendAPIKey, account_id: AhaSendAccountId) -> Self {
        Self {
            api_key,
            account_id,
        }
    }
    // https://ahasend.com/docs/send-api/send-email
    #[tracing::instrument]
    pub async fn send_email(&self, payload: EmailPayloadShape) -> anyhow::Result<()> {
        let reqwest_client = get_reqwest_client();
        let response = reqwest_client
            .post(format!(
                "https://api.ahasend.com/v2/accounts/{}/messages",
                &self.account_id.0.expose_secret()
            ))
            .bearer_auth(&self.api_key.0.expose_secret())
            .json(&payload)
            .send()
            .await
            .map_err(|e| {
                tracing::error!("Resp error {e:#?}");
                anyhow::anyhow!("{e:#?}")
            })?;
        tracing::debug!("Resp: {:?}", response.text().await.unwrap_or_default());
        Ok(())
    }
}

pub async fn ahasend_send_email(
    data: AhasendServiceData,
    email_data: TypicalEmail,
) -> anyhow::Result<()> {
    tracing::debug!("inside aha_send_send_email");
    let AhasendServiceData {
        sender_qualified_domain_name,
        api_key,
        account_id,
    } = data;
    let aha_send = { AhaSend::new(api_key, account_id) };

    let TypicalEmail {
        from_email_username_part,
        from_name,
        to_email_address,
        to_name,
        message_title,
        message_content,
    } = email_data;
    aha_send
        .send_email(EmailPayloadShape {
            sandbox: false,
            from: aha_send::Entity {
                email: format!("{from_email_username_part}@{sender_qualified_domain_name}"),
                name: from_name,
            },
            recipients: vec![Entity {
                email: to_email_address,
                name: to_name,
            }],
            subject: message_title,
            text_content: message_content.clone(),
            html_content: message_content,
        })
        .await
}
