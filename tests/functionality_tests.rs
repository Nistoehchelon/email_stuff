use email_sender::various_services::{self, TypicalEmail};

#[tokio::test]
pub async fn test_resend_sending() {
    let api_key = std::env::var("RESEND_API_KEY").expect("Pass a RESEND_API_KEY");
    let from_email_address = std::env::var("FROM_ADDR_TEST").expect("Pass a FROM_ADDR_TEST");
    let to_email_address = std::env::var("TO_EMAIL_TEST").expect("Pass a TO_EMAIL_TEST");
    various_services::resend_send_email(
        secrecy::SecretString::from(api_key),
        TypicalEmail {
            from_email_address,
            from_name: "Test Name".to_string(),
            to_email_address,
            to_name: "To name test".to_owned(),
            message_title: "Message from cargo test".to_owned(),
            message_content: "Awesome!".to_owned(),
        },
    )
    .await;
}
