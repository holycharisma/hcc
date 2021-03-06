use std::collections::HashMap;

// encoding scheme lovingly borrowed from the tari project
// encode u8 bytes into a 256 char map of emojis
// each emoji is 4 bytes... so this encoding scheme makes the xfer size 4X...
// but now we get cute picture representations of our binary data!

// scheme is only as consistent as this map... if this map changes, so do the encodings!
// just like the tari impl we use a luhn token at the end of the string so we can check the integrity before we interpret the bytes

const EMOJI: [char; 256] = [
    '๐ฆ', '๐คจ', '๐บ', '๐ฆ', '๐ค', '๐ท', '๐', '๐ค', '๐ค', '๐ฐ', '๐', '๐', '๐ป', '๐', '๐ฃ', '๐ง',
    '๐ ', '๐ค ', '๐ป', '๐', '๐', '๐ค', '๐', '๐งก', '๐คก', '๐คซ', '๐ผ', '๐ฅ', '๐ท', '๐ค', '๐คฏ', '๐ฅถ',
    '๐ถ', '๐', '๐ต', '๐ถ', '๐', '๐', '๐ค', '๐', '๐', '๐ถ', '๐', 'โ', '๐', '๐', '๐ฟ', '๐จ',
    '๐', '๐ฃ', '๐ค', '๐', '๐', '๐ฎ', '๐', '๐ข', '๐ฑ', '๐', '๐', '๐ท', '๐ช', '๐', '๐', '๐',
    '๐', '๐', '๐ฆ', '๐ข', '๐', '๐ฆ', '๐พ', '๐', '๐', '๐ฏ', '๐', '๐บ', '๐', '๐ท', '๐', '๐จ',
    '๐', '๐ ', '๐ธ', '๐', '๐ฉ', '๐ฐ', '๐ถ', '๐', '๐', '๐ซ', '๐ต', '๐ค', '๐ก', '๐ฅ', '๐คง', '๐พ',
    '๐ฐ', '๐', '๐คฒ', '๐ฅ', '๐', '๐ฏ', 'โ', '๐', '๐ธ', '๐ธ', '๐ง', 'โฝ', '๐', 'โ', '๐บ', '๐',
    '๐บ', '๐ง', '๐ฃ', '๐ค', '๐', '๐ท', '๐ฅ', '๐', '๐', '๐', '๐', '๐ฅ', '๐', '๐ซ', '๐', '๐ฑ',
    '๐ฃ', '๐', '๐ง', '๐', '๐น', '๐', '๐ผ', '๐', '๐ก', '๐ฝ', '๐', '๐จ', '๐ซ', '๐งข', '๐ค', '๐',
    '๐ซ', '๐ผ', '๐ป', '๐ฒ', '๐ป', '๐ช', '๐ฟ', '๐ง', '๐ฎ', '๐ญ', '๐', '๐ธ', '๐', '๐', '๐ต', '๐',
    '๐ช', '๐ง', '๐', '๐พ', '๐', '๐คธ', '๐ฑ', '๐', '๐ด', '๐ข', '๐', '๐ฝ', '๐', '๐บ', '๐', 'โฐ',
    '๐', '๐', '๐ฆ', 'โญ', '๐ฅ', '๐พ', '๐', '๐ฅ', '๐ฒ', '๐', '๐', '๐ธ', '๐ฅ', '๐ฟ', '๐', '๐',
    '๐ค', '๐', '๐ฆ', '๐ฟ', '๐ฆ', '๐', '๐ฌ', '๐งธ', '๐', '๐จ', '๐', '๐ค', '๐ฉ', '๐ต', '๐ผ', '๐ญ',
    '๐', '๐ฅฐ', 'โซ', '๐ง', '๐', '๐ค', '๐ฟ', '๐งฟ', '๐', '๐', '๐ณ', '๐', '๐ฆ', 'โพ', '๐คฐ', '๐น',
    '๐ฆ', '๐', '๐ง', '๐', '๐', '๐', '๐', '๐ช', '๐', '๐', '๐ ', '๐ฌ', '๐ต', '๐', '๐', '๐ฉ',
    '๐ฆ', '๐', '๐', '๐ธ', '๐', '๐', '๐ฆ', '๐ฌ', '๐ฅค', '๐น', '๐ผ', '๐พ', '๐ง', '๐ฑ', '๐ฎ', '๐ง ',
];

lazy_static! {
    static ref REVERSE_EMOJI: HashMap<char, usize> = {
        let mut m = HashMap::with_capacity(256);
        EMOJI.iter().enumerate().for_each(|(i, c)| {
            m.insert(*c, i);
        });
        assert_eq!(m.len(), EMOJI.len());
        m
    };
}

mod luhn {

    // source included from
    // from https://github.com/tari-project/tari/blob/95ac87db600fff7d6bc5d48459f144e6fce4ea3f/base_layer/common_types/src/luhn.rs

    pub fn valid(arr: &[usize], dict_len: usize) -> bool {
        if arr.len() < 2 {
            return false;
        }
        let cs = checksum(&arr[..arr.len() - 1], dict_len);
        cs == arr[arr.len() - 1]
    }

    pub fn checksum(arr: &[usize], dict_len: usize) -> usize {
        let (sum, _) = arr
            .iter()
            .rev()
            .fold((0usize, 2usize), |(sum, factor), digit| {
                let mut addend = factor * *digit;
                let factor = factor ^ 3;
                addend = (addend / dict_len) + addend % dict_len;
                (sum + addend, factor)
            });
        (dict_len - (sum % dict_len)) % dict_len
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct EmojiEncodedBytes(String);

impl EmojiEncodedBytes {
    pub fn emoji_checksum(emoji: &str) -> char {
        let indices = emoji.chars().map(|c| REVERSE_EMOJI.get(&c).unwrap());

        let idx_vec: Vec<usize> = indices.cloned().collect();

        let idx = luhn::checksum(&idx_vec, 256);

        EMOJI[idx]
    }

    pub fn blake_hash_to_secret(bytes: Vec<u8>) -> Vec<u8> {
        // reduce the 128 bytes of the blake hash into 32 bytes for our initial handshake key...
        // do not store this anywhere, just rely on this code to run
        // "middle-out" key extraction

        let as_utf8 = String::from_utf8(bytes.clone()).unwrap();
        let cool = as_utf8.chars();

        let mut xs: Vec<char> = cool.take(32).step_by(2).collect();
        let mut ys: Vec<char> = as_utf8.chars().skip(33).step_by(2).collect();

        ys.append(&mut xs);

        let emoji_str: String = ys.into_iter().collect();

        let checksum = EmojiEncodedBytes::emoji_checksum(&emoji_str);

        let signed = format!("{}{}", emoji_str, checksum);

        let e = EmojiEncodedBytes(signed);

        e.as_bytes()
    }

    pub fn from_bytes(bytes: &[u8]) -> Self {
        let mut vec = Vec::<usize>::new();
        bytes.iter().for_each(|b| vec.push((*b) as usize));
        let c = luhn::checksum(&vec, 256);
        vec.push(c as usize);
        let id = vec.iter().map(|b| EMOJI[*b]).collect();
        Self(id)
    }

    pub fn as_bytes(self) -> Vec<u8> {
        let emoji = self.0;

        let mut vec = Vec::<usize>::new();

        for c in emoji.chars() {
            let index = REVERSE_EMOJI.get(&c).unwrap();
            vec.push(*index);
        }

        assert!(luhn::valid(&vec, 256));

        vec.iter().take(vec.len() - 1).map(|s| *s as u8).collect()
    }
}

pub fn encode(bytes: &[u8]) -> String {
    EmojiEncodedBytes::from_bytes(bytes).0
}

pub fn decode(emojis: &str) -> Vec<u8> {
    let e = EmojiEncodedBytes(emojis.to_owned());
    e.as_bytes()
}

#[cfg(test)]
mod test {

    use crate::util::encryption::DeterministicEmojiEncrypt;

    use super::*;
    use orion::aead::{streaming::Nonce, SecretKey};

    #[test]
    fn test_emoji_byte_round_trip() {
        let hex = "ABCDEF1234567890";

        let hex_bytes = hex.as_bytes();

        let encoded_from_bytes = EmojiEncodedBytes::from_bytes(&hex_bytes);

        let expected_emoji = "๐๐ฆ๐ข๐๐ฆ๐พ๐ฃ๐ค๐๐๐ฎ๐๐ข๐ฑ๐๐๐ค";

        println!("Do emoji match?");
        assert_eq!(encoded_from_bytes.0, expected_emoji);
        assert_eq!(encoded_from_bytes.0, expected_emoji);

        let byte_decoded = encoded_from_bytes.as_bytes();

        println!("Does decoding match?");
        assert_eq!(hex_bytes, byte_decoded);

        assert_eq!(hex_bytes.len(), 16); // byte encoding
        assert_eq!(expected_emoji.len(), 68) // emoji encoding = 4 bytes per + 4 for checksum
    }

    #[test]
    fn emoji_checksum() {
        let checksum = EmojiEncodedBytes::emoji_checksum("๐๐ฆ๐ข๐๐ฆ๐พ๐ฃ๐ค๐๐๐ฎ๐๐ข๐ฑ๐๐");
        assert_eq!('๐ค', checksum)
    }

    #[test]
    fn test_blake_stuff() {
        let emoji_str = "๐จ๐ฆ๐ฒ๐ฆ๐ญ๐ต๐น๐๐ด๐ฆ๐๐ฒ๐ซ๐ถ๐๐๐ป๐ค๐ธ๐๐ข๐ท๐ธ๐จ๐๐ฃ๐ค๐งก๐ฉ๐ฆ๐ก๐บ๐๐ฉ๐๐ฃโ๐ฅ๐ง๐๐๐๐ฅฐ๐ผ๐๐ค๐ช๐๐๐ด๐ป๐ฏ๐๐พ๐คง๐ญ๐ฆ๐๐๐ข๐๐ฃ๐ช๐๐ฆ";
        let password = EmojiEncodedBytes::blake_hash_to_secret(emoji_str.as_bytes().to_owned());

        let key = SecretKey::from_slice(&password).unwrap();
        let secret = orion::aead::seal(&key, "secrets".as_bytes()).unwrap();

        let decrypted = String::from_utf8(orion::aead::open(&key, &secret).unwrap()).unwrap();

        assert_eq!(decrypted, "secrets");
    }

    #[test]
    fn test_emoji_deterministic_encryption_stuff() {
        let nonce_password = orion::pwhash::Password::generate(24).unwrap();
        let nonce_bytes = nonce_password.unprotected_as_bytes();

        let nonce_emoji = encode(nonce_bytes);
        println!("encoded nonce: {}", nonce_emoji);

        let plaintext = "i'm a secret";
        let plaintext_bytes = plaintext.as_bytes();

        let auth_password = orion::pwhash::Password::generate(32).unwrap();
        let auth_bytes = auth_password.unprotected_as_bytes();

        let auth_emoji = encode(auth_bytes);
        println!("encoded auth: {}", auth_emoji);

        let encrypted = DeterministicEmojiEncrypt::new(&auth_emoji, &nonce_emoji, &plaintext_bytes)
            .unwrap()
            .encrypted;

        let encrypted_2 =
            DeterministicEmojiEncrypt::new(&auth_emoji, &nonce_emoji, &plaintext_bytes)
                .unwrap()
                .encrypted;

        assert_eq!(encrypted, encrypted_2);
    }
}
