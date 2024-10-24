use std::str::FromStr as _;

use nostr_sdk::Keys;

use crate::error::Error;

pub fn load_keys_from_env() -> Result<Keys, Error> {
    let secret_key = std::env::var("NSEC_SECRET_KEY")
        .map_err(|_| Error::EnvError("NSEC_SECRET_KEY is not set".to_string()))?;
    Keys::from_str(&secret_key).map_err(|e| Error::EnvError(e.to_string()))
}

#[cfg(test)]
mod tests {
    use nostr_sdk::ToBech32;

    use crate::test::set_test_env;

    #[test]
    fn test_generate_random_keys() {
        let keys = super::Keys::generate();
        println!("nsec key: {:?}", keys.secret_key().to_bech32().unwrap());
    }

    #[test]
    fn test_load_keys_from_env() {
        set_test_env();
        let keys = super::load_keys_from_env().unwrap();
        println!("public key: {:?}", keys.public_key());
    }
}
