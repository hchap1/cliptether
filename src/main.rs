mod error;
mod security;
mod config;

use crate::config::load_config;
use crate::config::write_config;
use crate::error::Res;
use crate::security::load_key;

pub const DAEMON_NAME: &str = "cliptether";
pub const PORT: u16 = 12345;

use spiderweb_async::network::Server;

#[tokio::main]
async fn main() -> Res<()> {

    let mut config = load_config().await?;
    let key = match config.key_file.as_ref() {
        Some(keyfile) => load_key(keyfile.to_path_buf())?,
        None => {
            let keyfile = rfd::AsyncFileDialog::new()
                .set_title("Select 32B keyfile")
                .pick_file()
                .await;

            match keyfile {
                Some(keyfile) => config.key_file = Some(keyfile.path().to_path_buf()),
                None => panic!("Failed to select keyfile")
            };

            write_config(config.clone()).await?;
            load_key(config.key_file.as_ref().unwrap().to_path_buf()).await?
        }
    };
    
    let mut server = Server::build(DAEMON_NAME, PORT, None, 10, 10);

    Ok(())

}
