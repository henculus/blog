use argon2rs::{Argon2, defaults::*, Variant, verifier::Encoded};
use rand::{distributions::Alphanumeric, Rng, thread_rng};

pub trait HashablePassword {
    fn hash(&self) -> String;
    fn verify(&self, password: &String) -> bool;
}

impl HashablePassword for String {
    fn hash(&self) -> String {
        let salt: String = thread_rng()
            .sample_iter(&Alphanumeric)
            .take(10)
            .collect();

        let data_hash = Encoded::default2i(
            self.as_bytes(),
            salt.as_bytes(),
            b"",
            b"",
        ).to_u8();
        String::from_utf8(data_hash).unwrap()
    }

    fn verify(&self, password: &String) -> bool {
        match Encoded::from_u8(self.as_ref()) {
            Ok(hash) => hash.verify(password.as_ref()),
            Err(_) => false
        }
    }
}

mod tests {
    use super::HashablePassword;

    #[test]
    fn test_hash_verify() {
        let password = "qwerty".to_string();
        let hashed_password = password.hash();

        let wrong_hashed_password = "password".to_string().hash();

        assert!(hashed_password.verify(&password));
        assert!(!hashed_password.verify(&"hello".to_string()));
        assert!(!wrong_hashed_password.verify(&password));
    }
}