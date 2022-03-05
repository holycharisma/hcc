#![allow(dead_code, unused_imports)]

use serde::{Serialize, Deserialize};

use orion::kex::{SecretKey};
use orion::aead;

use super::emoji;

use wasm_bindgen::prelude::*;

macro_rules! expect_two {
    ($iter:expr) => {{
        let mut i = $iter;
        match (i.next(), i.next(), i.next()) {
            (Some(first), Some(second), None) => (first, second),
            _ => ("", ""),
        }
    }};
}

#[derive(Serialize, Deserialize, Debug)]
struct EncryptedKeyring {
    b: String
}

#[derive(Serialize, Deserialize, Debug)]
struct JsonClaims {
    exp: i32,
    iss: String,
    keyring: EncryptedKeyring,
    sid: String
}

#[derive(Serialize, Deserialize, Debug)]
struct TopSecretSharedKeyring {
    a: String,
    b: String,
    x: String,
    y: String
}


#[wasm_bindgen]
pub struct SharedKeyring {
    broadcast_secret: Vec<u8>,
    user_secret: Vec<u8>
}

#[wasm_bindgen]
impl SharedKeyring {
    #[wasm_bindgen(constructor)]
    pub fn new(decrypted_secrets: JsValue) -> SharedKeyring {
        let convert: TopSecretSharedKeyring = serde_wasm_bindgen::from_value(decrypted_secrets).unwrap();
        SharedKeyring { 
            broadcast_secret: emoji::decode(&convert.x),
            user_secret: emoji::decode(&convert.y)
         }
    }
    
    pub fn decrypt(&self, encrypted: &str) -> String {
        let secret = SecretKey::from_slice(&self.broadcast_secret).unwrap();

        let message_bytes = emoji::decode(encrypted);

        let bytes = aead::open(&secret, &message_bytes).unwrap();
        String::from_utf8(bytes).expect("invalid utf8")
    }

    pub fn decrypt_self(&self, encrypted: &str) -> String {
        let secret = SecretKey::from_slice(&self.user_secret).unwrap();

        let message_bytes = emoji::decode(&encrypted);

        let bytes = aead::open(&secret, &message_bytes).unwrap();
        String::from_utf8(bytes).expect("invalid utf8")
    }

    pub fn decrypt_header(&self, encrypted: &str) -> String {
        let secret = SecretKey::from_slice(&self.broadcast_secret).unwrap();

        let message_bytes = base64::decode(encrypted).unwrap();

        let bytes = aead::open(&secret, &message_bytes).unwrap();
        String::from_utf8(bytes).expect("invalid utf8")
    }
    
    pub fn encrypt(&self, plaintext: &str) -> String {
        let secret = SecretKey::from_slice(&self.user_secret).unwrap();
        let bytes = aead::seal(&secret, plaintext.as_bytes()).unwrap();
        emoji::encode(&bytes)
    }

    pub fn encrypt_header(&self, plaintext: &str) -> String {
        let secret = SecretKey::from_slice(&self.user_secret).unwrap();
        let bytes = aead::seal(&secret, plaintext.as_bytes()).unwrap();
        base64::encode_config(&bytes, base64::URL_SAFE_NO_PAD)
    }

    pub fn empty() -> SharedKeyring {
        Self {
            broadcast_secret: vec![],
            user_secret: vec![]
        }
    }
}


#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace=JSON)]
    pub fn parse(message: &str) -> JsValue; 

    pub fn atob(message: &str) -> String;
}


#[wasm_bindgen]
pub fn recv_claims(issuer: &str, csrf_signed: &str,) -> JsValue {

    let (_signature, message) = expect_two!(csrf_signed.rsplitn(2, '.'));
    let (_header, claims) = expect_two!(message.splitn(2, '.'));

    let try_decode = base64::decode_config(claims, base64::URL_SAFE_NO_PAD).unwrap();

    let extracted_json = String::from_utf8(try_decode).unwrap();

    let decoded = parse(&extracted_json);

    let claims_json: JsonClaims = serde_wasm_bindgen::from_value(decoded).unwrap();

    if claims_json.iss == issuer {
        let secret_slice = emoji::EmojiEncodedBytes::blake_hash_to_secret(claims_json.sid.as_bytes().to_owned());

        let message = emoji::decode(&claims_json.keyring.b);
        let secret: SecretKey = SecretKey::from_slice(&secret_slice).unwrap();
    
        let bytes = aead::open(&secret, &message).unwrap();
        let json = String::from_utf8(bytes).unwrap();
        parse(&json)
    } else {
        parse("{}")
    }
    
}