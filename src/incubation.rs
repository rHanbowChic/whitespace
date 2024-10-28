use std::collections::VecDeque;
use pbkdf2::pbkdf2_hmac;
use ring::aead::{Aad, BoundKey, LessSafeKey, Nonce, UnboundKey, AES_256_GCM};
use ring::rand::SecureRandom;
use std::str;

pub struct Incubator {
    mapping_cache: VecDeque<PageMapping>,
    enc_cache: VecDeque<PageMapping>,
}

impl Incubator {
    pub fn new() -> Incubator {
        Incubator{
            mapping_cache: VecDeque::new(),
            enc_cache: VecDeque::new(),
        }
    }
    fn u8_to_hex(data: &[u8]) -> String {
        data.iter()
            .map(|byte| format!("{:02x}", byte))
            .collect::<String>()
    }

    pub fn get_mapping(&mut self, host: &str, namespace: &str, page: &str) -> String {
        let page_name = page.to_string();
        let salt = host.to_string() + namespace;
        let digest = Incubator::get_digest(&*salt, &*page_name, &mut self.mapping_cache);

        Incubator::u8_to_hex(&digest)

    }

    pub fn encrypt_with_raw(&mut self, host: &str, namespace: &str, page: &str, text: &str) -> Vec<u8> {
        let text = text.as_bytes().to_vec();
/*
        text.extend_from_slice(&{
            let mut r = [0u8; 16];
            let rd = ring::rand::SystemRandom::new();
            rd.fill(&mut r);
            r
        });
 */


        let key = self.get_encryption_key(host, namespace, page);
        let unbound_key = UnboundKey::new(&AES_256_GCM, &key).expect("Incubator: invalid key");
        let less_safe_key = LessSafeKey::new(unbound_key);
        let nonce_bits = {
            let mut r = [0u8; 12];
            let rd = ring::rand::SystemRandom::new();
            rd.fill(&mut r).expect("Incubator: failed to fill nonce");
            r
        };
        let nonce = Nonce::assume_unique_for_key(nonce_bits);
        let aad = Aad::empty();
        let mut out = text.clone();
        less_safe_key.seal_in_place_append_tag(
            nonce,
            aad,
            &mut out
        ).expect("Incubator: encrypt failed");
        [nonce_bits.to_vec(), out].concat()


    }

    pub fn decrypt_with_raw(
        &mut self,
        host: &str,
        namespace: &str,
        page: &str, encrypted: &Vec<u8>
    ) -> String {
        let encrypted = encrypted.clone();

        let key = self.get_encryption_key(host, namespace, page);

        let unbound_key = UnboundKey::new(&AES_256_GCM, &key).expect("Incubator: invalid key");
        let less_safe_key = LessSafeKey::new(unbound_key);
        let nonce_bits_result = encrypted.iter().cloned().take(12).collect::<Vec<_>>()
            .try_into();
        let mut nonce_bits = [0u8; 12];
        match nonce_bits_result {
            Ok(bits) => { nonce_bits = bits; }
            Err(_) => { return "".to_string(); }
        }
        let nonce = Nonce::assume_unique_for_key(nonce_bits);
        let aad = Aad::empty();
        let mut out = encrypted.iter().cloned().skip(12).collect::<Vec<_>>();
        let r = less_safe_key.open_in_place(
            nonce,
            aad,
            &mut out[..]
        );
        match r {
            Ok(n) => String::from_utf8_lossy(&n).to_string(),
            Err(_) => "".to_string()
        }

    }

    fn get_encryption_key(&mut self, host: &str,namespace:&str, page: &str) -> Vec<u8> {
        let page_name = page.to_string();
        let salt = host.to_string() + namespace;
        let salt = salt + host;
        let key = Incubator::get_digest(&*salt, &*page_name, &mut self.enc_cache);
        key.to_vec()
    }

    pub fn get_encryption_key_hex(&mut self, host: &str, namespace:&str, page: &str) -> String {
        let key = self.get_encryption_key(host, namespace, page);
        Incubator::u8_to_hex(&key)
    }

    fn get_digest(salt: &str, page_name: &str, cache: &mut VecDeque<PageMapping>) -> [u8; 32] {
        let salt = salt.to_string();
        let page_name = page_name.to_string();
        let mut digest = [0u8; 32];
        let mut found_in_cache = false;
        for pm in &mut *cache {
            if pm.salt == salt && pm.page_name == page_name {
                digest = pm.mapping;
                found_in_cache = true;
                break;
            }
        }
        if !found_in_cache {
            pbkdf2_hmac::<sha2::Sha256>(
                page_name.as_bytes(),
                salt.as_bytes(),
                0x3fff,
                &mut digest[..]
            );

            cache.push_back({
                let pm = PageMapping{
                    page_name,
                    salt,
                    mapping: digest,
                };
                pm
            });

            if cache.len() > 0xff {
                cache.pop_front();
            }
        }
        digest
    }
}





struct PageMapping {
    page_name: String,
    salt: String,
    mapping: [u8; 32]
}