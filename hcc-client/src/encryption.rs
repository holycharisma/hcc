#![allow(dead_code, unused_imports)]

use serde::{Serialize, Deserialize};

use orion::kex::{SecretKey};
use orion::aead;

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

fn blake_hash_to_secret(bytes: Vec<u8>) -> Vec<u8> {

    // reduce the 128 bytes of the blake hash into 32 bytes for our initial handshake key...
    // do not store this anywhere, just rely on this code to run
    // "middle-out" key extraction

    assert_eq!(128, bytes.len());

    let bytes_head = bytes.clone().into_iter();
    let bytes_head_b = bytes_head.clone().skip(1);

    let bytes_tail = bytes.clone().into_iter().rev();
    let bytes_tail_b = bytes_tail.clone().skip(1);

    let heads = bytes_head.step_by(2).zip(
        bytes_head_b.step_by(2).rev()
    );

    let tails = bytes_tail.step_by(2).zip(
        bytes_tail_b.step_by(2).rev()
    );

    let heads_items = heads.take(8).flat_map(|x| vec![x.0, x.1]);

    let tails_items = tails.take(8).flat_map(|x| vec![x.0, x.1]);

    heads_items.rev().zip(tails_items).flat_map(|x| vec![x.0, x.1]).collect()
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
            broadcast_secret: hex::decode(convert.x).unwrap(),
            user_secret: hex::decode(convert.y).unwrap()
         }
    }
    
    pub fn decrypt(&self, encrypted: &str) -> String {
        let secret = SecretKey::from_slice(&self.broadcast_secret).unwrap();

        let message_bytes = hex::decode(encrypted.to_owned()).unwrap();

        let bytes = aead::open(&secret, &message_bytes).unwrap();
        String::from_utf8(bytes).expect("invalid utf8")
    }

    pub fn decrypt_self(&self, encrypted: &str) -> String {
        let secret = SecretKey::from_slice(&self.user_secret).unwrap();

        let message_bytes = hex::decode(encrypted.to_owned()).unwrap();

        let bytes = aead::open(&secret, &message_bytes).unwrap();
        String::from_utf8(bytes).expect("invalid utf8")
    }
    
    pub fn encrypt(&self, plaintext: &str) -> String {
        let secret = SecretKey::from_slice(&self.user_secret).unwrap();
        let bytes = aead::seal(&secret, plaintext.as_bytes()).unwrap();
        hex::encode(bytes)
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
pub fn recv_claims(issuer: &str, csrf_signed: &str, _pubkey_hex: &str) -> JsValue {

    let (_signature, message) = expect_two!(csrf_signed.rsplitn(2, '.'));
    let (_header, claims) = expect_two!(message.splitn(2, '.'));

    let decoded = parse(&atob(claims));

    let claims_json: JsonClaims = serde_wasm_bindgen::from_value(decoded).unwrap();

    if claims_json.iss == issuer {
        let secret_slice = blake_hash_to_secret(claims_json.sid.as_bytes().to_owned());

        let message = hex::decode(claims_json.keyring.b.to_owned()).unwrap();
        let secret: SecretKey = SecretKey::from_slice(&secret_slice).unwrap();
    
        let bytes = aead::open(&secret, &message).unwrap();
        let json = String::from_utf8(bytes).unwrap();
        parse(&json)
    } else {
        parse("{}")
    }
    
}