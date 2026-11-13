pub mod various_services;
use secrecy::SecretString;

#[derive(serde::Deserialize, Debug, Clone)]
pub struct EmailSenderConfig {
    pub enabled_services: Vec<EnabledService>,
}

#[derive(serde::Deserialize, Debug, Clone)]
pub struct ResendAPIKey(pub SecretString);
#[derive(serde::Deserialize, Debug, Clone)]
pub struct AhaSendAPIKey(pub SecretString);
#[derive(serde::Deserialize, Debug, Clone)]
pub struct AhaSendAccountId(pub SecretString);
#[derive(serde::Deserialize, Debug, Clone)]
pub struct LetterMintProjectToken(pub SecretString);
#[derive(serde::Deserialize, Debug, Clone)]
pub enum EnabledService {
    Resend {
        api_key: ResendAPIKey,
    },
    AhaSend {
        api_key: AhaSendAPIKey,
        account_id: AhaSendAccountId,
    },
    Lettermint {
        project_token: LetterMintProjectToken,
    },
}

#[derive(Debug, Clone)]
pub struct TypicalEmail {
    pub from_email_address: String,
    pub from_name: String,
    pub to_email_address: String,
    pub to_name: String,
    pub message_title: String,
    pub message_content: String,
}

use rand::seq::IteratorRandom;

#[derive(serde::Deserialize, Clone, Copy, Debug, strum_macros::EnumIter)]
pub enum ExternalEmailSendingService {
    Resend,
    AhaSend,
    Lettermint,
}

pub async fn send_email(
    sender_config: EmailSenderConfig,
    email_data: TypicalEmail,
) -> anyhow::Result<EmailSendingSuccess, EmailSendingError> {
    let Ok(service_i_wanna_use) = randomly_select_service(sender_config.enabled_services.clone())
    else {
        return Err(
            EmailSendingError::CouldNotSendTheEmailBecauseTheSenderServiceFailedForSomeReason,
        );
    };
    match service_i_wanna_use {
        EnabledService::Resend { api_key } => {
            various_services::resend_send_email(api_key.0.clone(), email_data).await
        }
        EnabledService::AhaSend {
            api_key,
            account_id,
        } => {
            various_services::aha_send_send_email(
                api_key.0.clone(),
                account_id.0.clone(),
                email_data,
            )
            .await
        }
        EnabledService::Lettermint { project_token } => {
            let _resp = various_services::lettermint_send_email(project_token, email_data).await;
        }
    }

    Ok(EmailSendingSuccess::EmailWasSentSuccessfully)
}

fn randomly_select_service(
    selected_services: Vec<EnabledService>,
) -> anyhow::Result<EnabledService, EmailSendingError> {
    let mut rng = rand::rng();
    match selected_services.iter().choose(&mut rng) {
        None => {
            return Err(EmailSendingError::CouldNotRandomlyPickSendingService);
        }
        Some(c) => {
            return Ok(c.clone());
        }
    };
}
#[derive(Debug, Clone)]
pub enum EmailSendingError {
    CouldNotSendTheEmailBecauseTheSenderServiceFailedForSomeReason,
    CouldNotRandomlyPickSendingService,
}
#[derive(Debug, Clone)]
pub enum EmailSendingSuccess {
    EmailWasSentSuccessfully,
}
