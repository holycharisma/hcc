
use domain::server_config::ServerConfig;
use orion::aead;
use orion::errors::UnknownCryptoError;
use orion::kex::{EphemeralServerSession, EphemeralClientSession, SecretKey};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct BroadcastEncryptedMessage {
    pub sender: String,
    pub message: String
}

impl BroadcastEncryptedMessage {
    pub fn decrypt(&self, secrets: &SharedKeyring) -> Result<String, UnknownCryptoError> {
        let secret_bytes = hex::decode(secrets.broadcast_secret.to_owned()).unwrap();
        let secret = SecretKey::from_slice(&secret_bytes)?;

        let message_bytes = hex::decode(self.message.to_owned()).unwrap();

        let bytes = aead::open(&secret, &message_bytes)?;
        let s = String::from_utf8(bytes).expect("invalid utf8");
        Ok(s)
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct UserEncryptedMessage {
    pub sender: String,
    pub message: String
}

impl UserEncryptedMessage {
    pub fn decrypt(&self, secrets: &SharedKeyring) -> Result<String, UnknownCryptoError> {
        let secret_bytes = hex::decode(secrets.user_secret.to_owned()).unwrap();
        let secret = SecretKey::from_slice(&secret_bytes)?;
        let message_bytes = hex::decode(self.message.to_owned()).unwrap();
        let bytes = aead::open(&secret, &message_bytes)?;
        let s = String::from_utf8(bytes).expect("invalid utf8");
        Ok(s)
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct EncryptedKeyring {
    b: String
}

#[derive(Serialize, Deserialize, Debug)]
struct TopSecretSharedKeyring {
    a: String,
    b: String,
    x: String,
    y: String
}

impl EncryptedKeyring {

    pub fn decrypt_with_secret(&self, secret_slice: &[u8]) -> Result<SharedKeyring, UnknownCryptoError> {
        let message = hex::decode(self.b.to_owned()).unwrap();
        let secret: SecretKey = SecretKey::from_slice(
            &secret_slice
        )?;
        let bytes = aead::open(&secret, &message)?;
        let json = String::from_utf8(bytes).unwrap();
        let shared_keyring: TopSecretSharedKeyring = serde_json::from_str(&json).expect("valid json");
        Ok(SharedKeyring {
            broadcast: shared_keyring.a,
            user: shared_keyring.b,
            broadcast_secret: shared_keyring.x,
            user_secret: shared_keyring.y
        })
    }

    pub fn decrypt_global(&self, config: &ServerConfig) -> Result<SharedKeyring, UnknownCryptoError> {
        self.decrypt_with_secret(
            &hex::decode(config.session_secret.to_owned()).unwrap()
        )
    }


    pub fn encrypt_with_secret(keyring: &SharedKeyring, secret_slice: &[u8])-> Result<EncryptedKeyring, UnknownCryptoError> {
        let secret: SecretKey = SecretKey::from_slice(
            &secret_slice
        )?;
        let shared_keyring = TopSecretSharedKeyring {
            a: keyring.broadcast.to_owned(),
            b: keyring.user.to_owned(),
            x: keyring.broadcast_secret.to_owned(),
            y: keyring.user_secret.to_owned()
        };
        let message = serde_json::to_string(&shared_keyring).expect("serialize");
        let bytes = aead::seal(&secret, &message.as_bytes())?;
        Ok(
            EncryptedKeyring {
                b: hex::encode(bytes)
            }
        )
    }

    pub fn encrypt_global(keyring: &SharedKeyring, config: &ServerConfig)-> Result<EncryptedKeyring, UnknownCryptoError> {
        EncryptedKeyring::encrypt_with_secret(keyring, &hex::decode(config.session_secret.to_owned()).unwrap())
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
    pub user_secret: String
}

impl SharedKeyring {
    /*
    pub async fn generate_password_hex() -> Result<String, UnknownCryptoError> {
        Ok(hex::encode(orion::pwhash::Password::generate(16)?.unprotected_as_bytes().to_owned()))
    }
    */

    pub async fn encrypt_broadcast(&self, plaintext: &str) -> Result<BroadcastEncryptedMessage, UnknownCryptoError> {
        let secret_bytes = hex::decode(self.broadcast_secret.to_owned()).unwrap();
        let secret = SecretKey::from_slice(&secret_bytes)?;
        let bytes = aead::seal(&secret, plaintext.as_bytes())?;
        Ok(BroadcastEncryptedMessage {
            sender: self.broadcast.to_owned(),
            message: hex::encode(bytes)
        })
    }

    pub async fn encrypt_user(&self, plaintext: &str) -> Result<UserEncryptedMessage, UnknownCryptoError> {
        let secret_bytes = hex::decode(self.user_secret.to_owned()).unwrap();
        let secret = SecretKey::from_slice(&secret_bytes)?;
        let bytes = aead::seal(&secret, plaintext.as_bytes())?;
        Ok(UserEncryptedMessage {
            sender: self.user.to_owned(),
            message: hex::encode(bytes)
        })
    }

    pub async fn from(session_server: EphemeralServerSession, session_client: EphemeralClientSession) -> Result<SharedKeyring, UnknownCryptoError> {

        let session_server_pub_key = session_server.public_key().clone();

        let session_client_pub_key = session_client.public_key().clone();
    
        let client_key_pair = session_client.establish_with_server(&session_server_pub_key)?;

        let server_identity = hex::encode(session_server_pub_key.to_bytes());
        let client_identity =  hex::encode(session_client_pub_key.to_bytes());

        let client_rx_and_server_tx = hex::encode(client_key_pair.receiving().unprotected_as_bytes());
        let client_tx_and_server_rx = hex::encode(client_key_pair.transport().unprotected_as_bytes());
        
        let bundle = SharedKeyring {
            broadcast: server_identity,
            user:  client_identity,
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
