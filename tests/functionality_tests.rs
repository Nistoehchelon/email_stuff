use email_stuff::{
    TypicalEmail,
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
    let from_email_address = std::env::var("FROM_EMAIL_TEST").expect("Pass a FROM_EMAIL_TEST");
    let to_email_address = std::env::var("TO_EMAIL_TEST").expect("Pass a TO_EMAIL_TEST");
    various_services::resend_send_email(
        secrecy::SecretString::from(api_key),
        TypicalEmail {
            from_email_address,
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
    let from_email_address = std::env::var("FROM_EMAIL_TEST").expect("Pass a FROM_EMAIL_TEST");
    let to_email_address = std::env::var("TO_EMAIL_TEST").expect("Pass a TO_EMAIL_TEST");

    various_services::aha_send_send_email(
        api_key,
        account_id,
        TypicalEmail {
            from_email_address,
            from_name: "Aha test admin".to_string(),
            to_email_address,
            to_name: "Aha receiver test".to_string(),
            message_title: "hey testing ahasend".to_string(),
            message_content: "hmhmhm".to_string(),
        },
    )
    .await;
}
