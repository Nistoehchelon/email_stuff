use secrecy::{ExposeSecret, SecretString};

use crate::{
    TypicalEmail,
    various_services::{aha_send, get_reqwest_client},
};

#[derive(serde::Serialize)]
struct Entity {
    email: String,
    name: String,
}

#[derive(serde::Serialize)]
pub struct EmailPayloadShape {
    sandbox: bool,
    from: Entity,
    recipients: Vec<Entity>,
    subject: String,
    text_content: String,
    html_content: String,
}
pub struct AhaSend {
    api_key: SecretString,
    account_id: SecretString,
}
impl AhaSend {
    pub fn new(api_key: SecretString, account_id: SecretString) -> Self {
        Self {
            api_key,
            account_id,
        }
    }
    // https://ahasend.com/docs/send-api/send-email
    pub async fn send_email(&self, payload: EmailPayloadShape) {
        let reqwest_client = get_reqwest_client();
        let response = {
            let resp = match reqwest_client
                .post(format!(
                    "https://api.ahasend.com/v2/accounts/{}/messages",
                    &self.account_id.expose_secret()
                ))
                .bearer_auth(&self.api_key.expose_secret())
                .json(&payload)
                .send()
                .await
            {
                Ok(resp) => resp,
                Err(error) => {
                    tracing::debug!("Resp error {error:?}");
                    return;
                }
            };
            resp
        };
        tracing::debug!("Resp: {:?}", response.text().await.unwrap_or_default());
    }
}

pub async fn aha_send_send_email(
    api_key: SecretString,
    account_id: SecretString,
    email_data: TypicalEmail,
) {
    let aha_send = { AhaSend::new(api_key, account_id) };
    let TypicalEmail {
        from_email_address,
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
                email: from_email_address,
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
