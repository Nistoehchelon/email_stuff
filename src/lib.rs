pub mod various_services;
use secrecy::SecretString;
use strum::IntoEnumIterator;

#[derive(serde::Deserialize, Debug)]
pub struct EmailSenderConfig {
    pub resend_api_key: SecretString,
    pub aha_send_api_key: SecretString,
    pub lettermint_apy_key: SecretString,
}
use rand::seq::IteratorRandom;

use crate::various_services::TypicalEmail; // rand 0.9

#[derive(serde::Deserialize, Clone, Copy, Debug, strum_macros::EnumIter)]
pub enum ExternalEmailSendingService {
    Resend,
    AhaSend,
    Lettermint,
}

pub async fn send_email(
    sender_config: EmailSenderConfig,
    from_email_address: &str,
    from_name: &str,
    message_title: &str,
    message_content: &str,
    to_email_address: &str,
    to_name: &str,
) -> anyhow::Result<EmailSendingSuccess, EmailSendingError> {
    let Ok(service_i_wanna_use) = randomly_select_service() else {
        return Err(
            EmailSendingError::CouldNotSendTheEmailBecauseTheSenderServiceFailedForSomeReason,
        );
    };
    match service_i_wanna_use {
        ExternalEmailSendingService::Resend => {
          various_services::resend_send_email(
                sender_config.resend_api_key.clone(),
                TypicalEmail {
                    from_email_address: from_email_address.to_owned(),
                    from_name: from_name.to_owned(),
                    to_email_address: to_email_address.to_owned(),
                    to_name: to_name.to_owned(),
                    message_title: message_title.to_owned(),
                    message_content: message_content.to_owned(),
                },
            )
            .await
        }
        _ => {}
    }

    Ok(EmailSendingSuccess::EmailWasSentSuccessfully)
}

fn randomly_select_service() -> anyhow::Result<ExternalEmailSendingService, EmailSendingError> {
    let mut rng = rand::rng();
    match ExternalEmailSendingService::iter().choose(&mut rng) {
        None => {
            return Err(EmailSendingError::CouldNotRandomlyPickSendingService);
        }
        Some(c) => {
            return Ok(c);
        }
    };
}
pub enum EmailSendingError {
    CouldNotSendTheEmailBecauseTheSenderServiceFailedForSomeReason,
    CouldNotRandomlyPickSendingService,
}
pub enum EmailSendingSuccess {
    EmailWasSentSuccessfully,
}
