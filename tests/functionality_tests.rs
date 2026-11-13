use email_stuff::{
    AhasendServiceData, ResendAPIKey, ResendServiceData, TypicalEmail,
    various_services::{self},
};
use secrecy::SecretString;
fn setup_tracing() {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();
}

#[tokio::test]
pub async fn test_resend_sending() {
    setup_tracing();
    dotenvy::dotenv().ok();
    let api_key = std::env::var("RESEND_API_KEY").expect("Pass a RESEND_API_KEY");
    let from_email_username_part = std::env::var("FROM_EMAIL_USERNAME_PART_TEST")
        .expect("Pass a FROM_EMAIL_USERNAME_PART_TEST");
    let sender_fqdn = std::env::var("SENDER_QUALIFIED_DOMAIN_NAME")
        .expect("Pass a FROM_EMAIL_USERNAME_PART_TEST");
    let to_email_address = std::env::var("TO_EMAIL_TEST").expect("Pass a TO_EMAIL_TEST");
    various_services::resend_send_email(
        ResendServiceData {
            sender_qualified_domain_name: email_stuff::SenderQualifiedDomain(sender_fqdn),
            api_key: ResendAPIKey::from(api_key),
        },
        TypicalEmail {
            from_email_username_part,
            from_name: "Test Name".to_string(),
            to_email_address,
            to_name: "To name test".to_owned(),
            message_title: "Message from cargo test 2".to_owned(),
            message_content: "Awesome 2!".to_owned(),
        },
    )
    .await;
}

#[tokio::test]
pub async fn test_aha_send_sending() {
    setup_tracing();
    println!("gg");
    tracing::debug!("hhh");

    dotenvy::dotenv().ok();
    let api_key =
        SecretString::from(std::env::var("AHA_SEND_API_KEY").expect("Pass a AHA_SEND_API_KEY"));
    let account_id = SecretString::from(
        std::env::var("AHA_SEND_ACCOUNT_ID").expect("Pass a AHA_SEND_ACCOUNT_ID"),
    );
    let sender_email_fqdn = std::env::var("SENDER_EMAIL_FQDN").expect("Pass a SENDER_EMAIL_FQDN");
    let sender_email_username =
        std::env::var("SENDER_EMAIL_USERNAME").expect("Pass a FROM_EMAIL_USERNAME");
    let to_email_address = std::env::var("TO_EMAIL_TEST").expect("Pass a TO_EMAIL_TEST");

    various_services::ahasend_send_email(
        AhasendServiceData {
            sender_qualified_domain_name: email_stuff::SenderQualifiedDomain(sender_email_fqdn),
            api_key: email_stuff::AhaSendAPIKey::from(api_key),
            account_id: email_stuff::AhaSendAccountId::from(account_id),
        },
        TypicalEmail {
            from_email_username_part: sender_email_username,
            from_name: "Aha test admin".to_string(),
            to_email_address,
            to_name: "Aha receiver test".to_string(),
            message_title: "hey testing ahasend".to_string(),
            message_content: "hmhmhm".to_string(),
        },
    )
    .await;
}

#[tokio::test]
pub async fn test_lettermint_sending() {
    dotenvy::dotenv().ok();
    setup_tracing();

    let project_token =
        std::env::var("LETTERMINT_PROJECT_TOKEN").expect("Pass a LETTERMINT_PROJECT_TOKEN");
    let sender_email_fqdn = std::env::var("SENDER_EMAIL_FQDN").expect("Pass a SENDER_EMAIL_FQDN");
    let sender_email_username =
        std::env::var("SENDER_EMAIL_USERNAME").expect("Pass a FROM_EMAIL_USERNAME");
    let to_email_address = std::env::var("TO_EMAIL_TEST").expect("Pass a TO_EMAIL_TEST");

    let res = various_services::lettermint_send_email(
        email_stuff::LettermintServiceData {
            sender_qualified_domain_name: email_stuff::SenderQualifiedDomain(sender_email_fqdn),
            project_token: project_token.into(),
        },
        TypicalEmail {
            from_email_username_part: sender_email_username,
            from_name: "Aha test admin".to_string(),
            to_email_address,
            to_name: "Aha receiver test".to_string(),
            message_title: "hey testing ahasend".to_string(),
            message_content: "hmhmhm".to_string(),
        },
    )
    .await;
    tracing::debug!("{res:?}");
}
