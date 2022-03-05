use domain::server_config::ServerConfig;
use orion::aead;
use orion::errors::UnknownCryptoError;
use orion::kex::{EphemeralClientSession, EphemeralServerSession, SecretKey};
use serde::{Deserialize, Serialize};

use super::emoji;

pub async fn encrypt_str_emoji(
    body: &str,
    secrets: &SharedKeyring,
) -> Result<String, UnknownCryptoError> {
    let message = secrets.encrypt_broadcast_emoji(body).await?;
    Ok(message.message)
}


pub async fn encrypt_str_base64(
    body: &str,
    secrets: &SharedKeyring,
) -> Result<String, UnknownCryptoError> {
    let message = secrets.encrypt_broadcast_base64(body).await?;
    Ok(message.message)
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ServerEncryptedEmojiMessage {
    pub sender: String,
    pub message: String,
}

impl ServerEncryptedEmojiMessage {
    pub fn decrypt(&self, secrets: &SharedKeyring) -> Result<String, UnknownCryptoError> {
        let secret_bytes = emoji::decode(&secrets.broadcast_secret);
        let secret = SecretKey::from_slice(&secret_bytes)?;

        let message_bytes = emoji::decode(&self.message);

        let bytes = aead::open(&secret, &message_bytes)?;
        let s = String::from_utf8(bytes).expect("invalid utf8");
        Ok(s)
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct UserEncryptedEmojiMessage {
    pub sender: String,
    pub message: String,
}

impl UserEncryptedEmojiMessage {
    pub fn decrypt(&self, secrets: &SharedKeyring) -> Result<String, UnknownCryptoError> {
        let secret_bytes = emoji::decode(&secrets.user_secret);
        let secret = SecretKey::from_slice(&secret_bytes)?;
        let message_bytes = emoji::decode(&self.message);
        let bytes = aead::open(&secret, &message_bytes)?;
        let s = String::from_utf8(bytes).expect("invalid utf8");
        Ok(s)
    }
}



pub struct ServerEncryptedBase64Message {
    pub message: String
}

impl ServerEncryptedBase64Message {
    pub fn decrypt(&self, secrets: &SharedKeyring) -> Result<String, UnknownCryptoError> {
        let secret_bytes = emoji::decode(&secrets.broadcast_secret);
        let secret = SecretKey::from_slice(&secret_bytes)?;

        let message_bytes = base64::decode_config(&self.message, base64::URL_SAFE_NO_PAD).unwrap();
        let bytes = aead::open(&secret, &message_bytes)?;
        let s = String::from_utf8(bytes).expect("invalid utf8");
        Ok(s)
    }
}


pub struct UserEncryptedBase64Message {
    pub message: String
}

impl UserEncryptedBase64Message {
    pub fn decrypt(&self, secrets: &SharedKeyring) -> Result<String, UnknownCryptoError> {
        let secret_bytes = emoji::decode(&secrets.user_secret);
        let secret = SecretKey::from_slice(&secret_bytes)?;

        let message_bytes = base64::decode_config(&self.message, base64::URL_SAFE_NO_PAD).unwrap();
        let bytes = aead::open(&secret, &message_bytes)?;
        let s = String::from_utf8(bytes).expect("invalid utf8");
        Ok(s)
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct EncryptedKeyring {
    b: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct TopSecretSharedKeyring {
    a: String,
    b: String,
    x: String,
    y: String,
}

impl EncryptedKeyring {
    pub fn decrypt_with_secret(
        &self,
        secret_slice: &[u8],
    ) -> Result<SharedKeyring, UnknownCryptoError> {
        let message = emoji::decode(&self.b);
        let secret: SecretKey = SecretKey::from_slice(&secret_slice)?;
        let bytes = aead::open(&secret, &message)?;
        let json = String::from_utf8(bytes).unwrap();
        let shared_keyring: TopSecretSharedKeyring =
            serde_json::from_str(&json).expect("valid json");
        Ok(SharedKeyring {
            broadcast: shared_keyring.a,
            user: shared_keyring.b,
            broadcast_secret: shared_keyring.x,
            user_secret: shared_keyring.y,
        })
    }

    pub fn decrypt_global(
        &self,
        config: &ServerConfig,
    ) -> Result<SharedKeyring, UnknownCryptoError> {
        self.decrypt_with_secret(&emoji::decode(&config.session_secret))
    }

    pub fn encrypt_with_secret(
        keyring: &SharedKeyring,
        secret_slice: &[u8],
    ) -> Result<EncryptedKeyring, UnknownCryptoError> {
        let secret: SecretKey = SecretKey::from_slice(&secret_slice)?;
        let shared_keyring = TopSecretSharedKeyring {
            a: keyring.broadcast.to_owned(),
            b: keyring.user.to_owned(),
            x: keyring.broadcast_secret.to_owned(),
            y: keyring.user_secret.to_owned(),
        };
        let message = serde_json::to_string(&shared_keyring).expect("serialize");
        let bytes = aead::seal(&secret, &message.as_bytes())?;
        Ok(EncryptedKeyring {
            b: emoji::encode(&bytes),
        })
    }

    pub fn encrypt_global(
        keyring: &SharedKeyring,
        config: &ServerConfig,
    ) -> Result<EncryptedKeyring, UnknownCryptoError> {
        EncryptedKeyring::encrypt_with_secret(keyring, &emoji::decode(&config.session_secret))
    }
}

#[derive(Clone)]
pub struct SharedKeyring {
    // how do we get some sort of forward secrecy? or post-compromise security?
    // rotate your keys, orion wants these to be single use keys...

    // good security practice dictates you throw these away frequently
    // we store them on our session and rely on browser http only cookie security
    pub broadcast: String,
    pub user: String,

    pub broadcast_secret: String,
    pub user_secret: String,
}

impl SharedKeyring {
    // todo: this should probably be wired up into some kind of cargo run utility...
    // since it needs to go inside your .env file for the program to work
    pub async fn generate_password_emoji() -> Result<String, UnknownCryptoError> {
        Ok(emoji::encode(
            orion::pwhash::Password::generate(32)?.unprotected_as_bytes(),
        ))
    }



    pub async fn encrypt_broadcast_base64(
        &self,
        plaintext: &str,
    ) -> Result<ServerEncryptedBase64Message, UnknownCryptoError> {
        let secret_bytes = emoji::decode(&self.broadcast_secret);
        let secret = SecretKey::from_slice(&secret_bytes)?;
        let bytes = aead::seal(&secret, plaintext.as_bytes())?;
        Ok(ServerEncryptedBase64Message {
            message: base64::encode(&bytes),
        })
    }

    pub async fn encrypt_user_base64(
        &self,
        plaintext: &str,
    ) -> Result<UserEncryptedBase64Message, UnknownCryptoError> {
        let secret_bytes = emoji::decode(&self.user_secret);
        let secret = SecretKey::from_slice(&secret_bytes)?;
        let bytes = aead::seal(&secret, plaintext.as_bytes())?;
        Ok(UserEncryptedBase64Message {
            message: base64::encode(&bytes),
        })
    }


    pub async fn encrypt_broadcast_emoji(
        &self,
        plaintext: &str,
    ) -> Result<ServerEncryptedEmojiMessage, UnknownCryptoError> {
        let secret_bytes = emoji::decode(&self.broadcast_secret);
        let secret = SecretKey::from_slice(&secret_bytes)?;
        let bytes = aead::seal(&secret, plaintext.as_bytes())?;
        Ok(ServerEncryptedEmojiMessage {
            sender: self.broadcast.to_owned(),
            message: emoji::encode(&bytes),
        })
    }

    pub async fn encrypt_user_emoji(
        &self,
        plaintext: &str,
    ) -> Result<UserEncryptedEmojiMessage, UnknownCryptoError> {
        let secret_bytes = emoji::decode(&self.user_secret);
        let secret = SecretKey::from_slice(&secret_bytes)?;
        let bytes = aead::seal(&secret, plaintext.as_bytes())?;
        Ok(UserEncryptedEmojiMessage {
            sender: self.user.to_owned(),
            message: emoji::encode(&bytes),
        })
    }

    pub async fn from(
        session_server: EphemeralServerSession,
        session_client: EphemeralClientSession,
    ) -> Result<SharedKeyring, UnknownCryptoError> {
        let session_server_pub_key = session_server.public_key().clone();

        let session_client_pub_key = session_client.public_key().clone();

        let client_key_pair = session_client.establish_with_server(&session_server_pub_key)?;

        let server_identity = emoji::encode(&session_server_pub_key.to_bytes());
        let client_identity = emoji::encode(&session_client_pub_key.to_bytes());

        let client_rx_and_server_tx =
            emoji::encode(&client_key_pair.receiving().unprotected_as_bytes());
        let client_tx_and_server_rx =
            emoji::encode(&client_key_pair.transport().unprotected_as_bytes());

        let bundle = SharedKeyring {
            broadcast: server_identity,
            user: client_identity,
            broadcast_secret: client_rx_and_server_tx,
            user_secret: client_tx_and_server_rx,
        };

        Ok(bundle)
    }

    pub async fn new() -> Result<SharedKeyring, UnknownCryptoError> {
        let server_session = EphemeralServerSession::new()?;
        let client_session = EphemeralClientSession::new()?;
        SharedKeyring::from(server_session, client_session).await
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[async_std::test]
    async fn test_some_basic_password_gen() {

        let password = SharedKeyring::generate_password_emoji().await;

        println!("{:?}", password);
    }
}
