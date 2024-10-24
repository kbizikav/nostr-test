use dialoguer::Input;
use nostr_sdk::prelude::*;
use utils::load_keys_from_env;

pub mod error;
pub mod utils;

#[cfg(test)]
pub mod test;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv::dotenv().ok();

    let my_keys = load_keys_from_env()?;
    println!("public key: {}", my_keys.public_key().to_bech32()?);

    let client = Client::new(&my_keys);
    client.add_relay("wss://relay.damus.io").await?;
    client.add_read_relay("wss://relay.nostr.info").await?;
    client.connect().await;

    loop {
        let input: String = Input::new()
            .with_prompt("message?")
            .interact_text()
            .unwrap();
        client.publish_text_note(&input, []).await?;
    }
}
