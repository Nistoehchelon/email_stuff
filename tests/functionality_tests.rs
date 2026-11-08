use email_sender::{TypicalEmail, various_services::{self}};

#[tokio::test]
pub async fn test_resend_sending() {
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
