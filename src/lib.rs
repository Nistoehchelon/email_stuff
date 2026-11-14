pub mod various_services;
use secrecy::SecretString;

#[derive(serde::Deserialize, Debug, Clone)]
pub struct EmailSenderConfig {
    pub enabled_services: Vec<EnabledService>,
}

#[derive(serde::Deserialize, derive_more::Debug, derive_more::Deref, derive_more::From, Clone)]
#[from(forward)]
pub struct ResendAPIKey(pub SecretString);
#[derive(serde::Deserialize, derive_more::Debug, derive_more::Deref, derive_more::From, Clone)]
#[from(forward)]
pub struct AhaSendAPIKey(pub SecretString);
#[derive(serde::Deserialize, derive_more::Debug, derive_more::Deref, derive_more::From, Clone)]
#[from(forward)]
pub struct AhaSendAccountId(pub SecretString);
#[derive(serde::Deserialize, derive_more::Debug, derive_more::Deref, derive_more::From, Clone)]
#[from(forward)]
pub struct LetterMintProjectToken(pub SecretString);

#[derive(serde::Deserialize, Debug, Clone)]
pub struct ResendServiceData {
    pub sender_qualified_domain_name: SenderQualifiedDomain,
    pub api_key: ResendAPIKey,
}
#[derive(serde::Deserialize, Debug, Clone)]
pub struct AhasendServiceData {
    pub sender_qualified_domain_name: SenderQualifiedDomain,
    pub api_key: AhaSendAPIKey,
    pub account_id: AhaSendAccountId,
}
#[derive(serde::Deserialize, Debug, Clone)]
pub struct LettermintServiceData {
    pub sender_qualified_domain_name: SenderQualifiedDomain,
    pub project_token: LetterMintProjectToken,
}

#[derive(serde::Deserialize, Debug, Clone)]
pub enum EnabledService {
    Resend(ResendServiceData),
    AhaSend(AhasendServiceData),
    Lettermint(LettermintServiceData),
}
#[derive(serde::Deserialize, Debug, Clone, derive_more::Display)]
pub struct SenderQualifiedDomain(pub String); // example: supersend.mysite.com

#[derive(Debug, Clone)]
pub struct TypicalEmail {
    pub from_email_username_part: String,
    pub from_name: String,
    pub to_email_address: String,
    pub to_name: String,
    pub message_title: String,
    pub message_content: String,
}

use rand::seq::IteratorRandom;

#[tracing::instrument]
pub async fn send_email(
    sender_config: EmailSenderConfig,
    email_data: TypicalEmail,
) -> anyhow::Result<EmailSendingSuccess> {
    let Ok(service_i_wanna_use) = randomly_select_service(sender_config.enabled_services.clone())
    else {
        return Err(anyhow::anyhow!("Could not select seervice"));
    };
    tracing::debug!("Choosen service: {service_i_wanna_use:#?}");
    match service_i_wanna_use {
        EnabledService::Resend(data) => {
            various_services::resend_send_email(data, email_data).await?
        }
        EnabledService::AhaSend(data) => {
            various_services::ahasend_send_email(data, email_data).await?
        }
        EnabledService::Lettermint(data) => {
            let _resp = various_services::lettermint_send_email(data, email_data).await;
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
