#[tokio::main(flavor = "multi_thread")]
async fn main() -> anyhow::Result<()> {
    if server::will_show_version_then_would_like_to_terminate_the_program(VERSION) {
        return Ok(());
    }
    dotenvy::dotenv().ok();
    let email_sender_config = config_reader::read_config::<EmailSenderConfig>(PathBuf::from(".env"));
    let server_config = server_config::ServerConfig::new(
        email_sender_config.ip.to_string(),
        email_sender_config.port,
        email_sender_config.rust_log_directory_path.clone(),
    );
    tracing::debug!("{email_sender_config:?}");
    Ok(())
}

use std::path::PathBuf;

use email_sender::EmailSenderConfig;
use git_version::git_version;

pub const VERSION: &str = git_version!(args = ["--tags", "--abbrev=0"]); /*  increase when I want to compare which one I am running on prod 
just --tags shows some sort of hash at the end.
--abbrev being set to 0 is necessary to return only the tag.
*/
