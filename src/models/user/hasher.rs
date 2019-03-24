use argon2rs::verifier::Encoded;
use rand::{distributions::Alphanumeric, Rng, thread_rng};

use crate::models::error::ModelError;

pub trait HashablePassword {
    fn hash(&self) -> String;
    fn verify_hash(&self, password: &String) -> Result<(), ModelError>;
}

impl HashablePassword for String {
    fn hash(&self) -> String {
        let salt: String = thread_rng().sample_iter(&Alphanumeric).take(10).collect();

        let data_hash = Encoded::default2i(self.as_bytes(), salt.as_bytes(), b"", b"").to_u8();
        String::from_utf8(data_hash).unwrap()
    }

    fn verify_hash(&self, password: &String) -> Result<(), ModelError> {
        match Encoded::from_u8(self.as_ref()) {
            Ok(hash) => {
                if hash.verify(password.as_ref()) {
                    Ok(())
                } else {
                    Err(
                        ModelError::InvalidCredentials(None)
                    )
                }
            }
            Err(_) => Err(
                ModelError::DatabaseError(Some("Cannot read hash from database".to_string()))
            ),
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

        assert!(hashed_password.verify_hash(&password).is_ok());
        assert!(hashed_password.verify_hash(&"hello".to_string()).is_err());
        assert!(wrong_hashed_password.verify_hash(&password).is_err());
    }
}
