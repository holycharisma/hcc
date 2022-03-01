
use jsonwebtokens::{encode, Algorithm, AlgorithmID, Verifier};

use tide::prelude::*;

/*

// begin JWT auth stuff

// https://blog.logrocket.com/how-to-secure-a-rest-api-using-jwt-7efd83e71432/


#from https://gist.github.com/ygotthilf/baa58da5c3dd1f69fae9

ssh-keygen -t rsa -b 4096 -m PEM -f jwtRS256.key
# Don't add passphrase
openssl rsa -in jwtRS256.key -pubout -outform PEM -out jwtRS256.key.pub
cat jwtRS256.key
cat jwtRS256.key.pub

*/


#[derive(Clone)]
pub struct JsonWebTokenUtil {
    pub secrets: JsonWebTokenSecrets,
    pub issuer: String,
    pub expiry_duration_millis: i64,
}

impl JsonWebTokenUtil {

    
    pub fn verify_auth_token(
        self: &JsonWebTokenUtil, token_str: &str, email: &str) -> Result<serde_json::value::Value, jsonwebtokens::error::Error> {
            
        let pem_data = &self.secrets.pub_key_pem_data[..];

        let alg = Algorithm::new_rsa_pem_verifier(AlgorithmID::RS256, pem_data)?;

        let verifier = Verifier::create().issuer(&self.issuer).string_equals("email", email).build()?;

        verifier.verify(&token_str, &alg)
    }

    pub fn sign_auth_token(self: &JsonWebTokenUtil, email: &str) -> Result<String, jsonwebtokens::error::Error> {
        let pem_data = &self.secrets.key_pem_data[..];

        let alg = Algorithm::new_rsa_pem_signer(AlgorithmID::RS256, pem_data)?;
        let header = json!({ "alg": alg.name() });
        let now = chrono::Utc::now().timestamp();
        let twentyfour_hr_millis = self.expiry_duration_millis;
        let exp = now + twentyfour_hr_millis;
        let claims = json!({ "iss": &self.issuer, "exp": exp, "email": &email });
        
        encode(&header, &claims, &alg)
    }

    pub fn verify_csrf_token(
        self: &JsonWebTokenUtil,
        token_str: &str,
        expected_session_id: &str
    ) -> Result<serde_json::value::Value, jsonwebtokens::error::Error> {

        let pem_data = &self.secrets.pub_key_pem_data[..];

        let alg = Algorithm::new_rsa_pem_verifier(AlgorithmID::RS256, pem_data)?;

        let verifier = Verifier::create().issuer(&self.issuer).string_equals("sid", expected_session_id).build()?;

        verifier.verify(&token_str, &alg)
    }

    pub fn sign_csrf_token(self: &JsonWebTokenUtil, session_id: &str) -> Result<String, jsonwebtokens::error::Error> {
        let pem_data = &self.secrets.key_pem_data[..];

        let alg = Algorithm::new_rsa_pem_signer(AlgorithmID::RS256, pem_data)?;
        let header = json!({ "alg": alg.name() });
        let now = chrono::Utc::now().timestamp();
        let twentyfour_hr_millis = self.expiry_duration_millis;
        let exp = now + twentyfour_hr_millis;
        let claims = json!({ "iss": &self.issuer, "exp": exp, "sid": session_id });
        
        encode(&header, &claims, &alg)
    }
}


#[derive(Clone)]
pub struct JsonWebTokenSecrets {
    key_pem_data: Vec<u8>,
    pub_key_pem_data: Vec<u8>,
}

impl JsonWebTokenSecrets {

    pub fn read_keys(key_path: &str, pubkey_path: &str) -> JsonWebTokenSecrets {
        let key_bytes = std::fs::read(key_path).expect("Unable to load RSA key file.");
        let pubkey_bytes = std::fs::read(pubkey_path).expect("Unable to load RSA public key file.");

        JsonWebTokenSecrets {
            key_pem_data: key_bytes,
            pub_key_pem_data: pubkey_bytes,
        }
    }
}
