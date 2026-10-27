mod error;
mod security;
mod config;

use crate::config::load_config;
use crate::config::write_config;
use crate::error::Res;
use crate::security::load_key;

pub const DAEMON_NAME: &str = "cliptether";
pub const PORT: u16 = 12345;

use bytes::Bytes;
use spiderweb_async::network::Server;
use clipboard_master::CallbackResult;
use clipboard_master::ClipboardHandler;
use clipboard_master::Master;
use async_channel::Sender;
use async_channel::Receiver;

struct Handler {
    sender: Sender<()>
}

impl ClipboardHandler for Handler {
    fn on_clipboard_change(&mut self) -> CallbackResult {
        let _ = self.sender.send_blocking(());
        CallbackResult::Next
    }
}

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

    let (callback_sender, callback_receiver) = async_channel::bounded(10);
    tokio::task::spawn_blocking(|| Master::new(Handler { sender: callback_sender }).expect("Failed to start clipboard callback").run());

    let sender = server.detach_send_all();

    tokio::spawn(async {

        while let Ok(()) = callback_receiver.recv().await {
            // grab the clipboard content, serialize

            let packet = Bytes::from_static(&[0_u8]); // placeholder
            let encrypted = crate::security::encrypt(key, &packet);
            let _ = sender.send(encrypted).await;
        }

    });

    while let Ok(packet) = server.incoming.recv().await {
        let decrypted = match crate::security::decrypt(key, &packet.0) {
            Some(trusted) => trusted,
            None => continue
        };

        // convert back into a clipboard object, write to clipboard
    }

    Ok(())

}
