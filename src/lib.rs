use rand::{rngs::ThreadRng, seq::IndexedRandom};
use std::net::IpAddr;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[derive(serde::Deserialize, Debug)]
pub struct EmailSenderConfig {
    pub ip: IpAddr,
    pub port: u64,
    pub rust_log_directory_path: String,
    pub api_key: String,
}
use rand::seq::IteratorRandom; // rand 0.9

#[derive(serde::Deserialize, Clone, Copy, Debug, strum_macros::EnumIter)]
pub enum ExternalEmailSendingService {
    Resend,
    AhaSend,
}

pub async fn send_email(
    from_email_address: &str,
    from_name: &str,
    message_title: &str,
    message_content: &str,
    recipient_email_address: &str,
    recipient_name: &str,
) -> anyhow::Result<EmailSendingSuccess, EmailSendingError> {
    let Ok(service_i_wanna_use) = randomly_select_service() else {
        return Err(EmailSendingError::CouldNotSendTheEmailBecauseTheSenderServiceFailedForSomeReason);
    };

    Ok(EmailSendingSuccess::EmailWasSentSuccessfully)
}

fn randomly_select_service()
-> anyhow::Result<ExternalEmailSendingService, RandomlySelectingServiceError> {
    let mut rng = rand::rng();
    match ExternalEmailSendingService::iter().choose(&mut rng) {
        None => {
            return Err(RandomlySelectingServiceError::CouldNotRandomlyPickSendingService);
        }
        Some(c) => {
            return Ok(c);
        }
    };
}
pub enum EmailSendingError {
    CouldNotSendTheEmailBecauseTheSenderServiceFailedForSomeReason,
    CouldNotRandomlyPickSendingService
}
pub enum EmailSendingSuccess {
    EmailWasSentSuccessfully,
}
