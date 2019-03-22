use argon2rs::{Argon2, defaults::*, Variant, verifier::Encoded};
use rand::{distributions::Alphanumeric, Rng, thread_rng};

use crate::models::error::{ModelError, ModelErrorKind};

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
                        ModelError {
                            kind: ModelErrorKind::InvalidCredentials,
                            message: "Invalid credentials".to_string(),
                        }
                    )
                }
            }
            Err(_) => Err(
                ModelError {
                    kind: ModelErrorKind::OperationError,
                    message: "Cannot read hash from database".to_string(),
                }
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

        assert!(hashed_password.verify_hash(&password));
        assert!(!hashed_password.verify_hash(&"hello".to_string()));
        assert!(!wrong_hashed_password.verify_hash(&password));
    }
}
