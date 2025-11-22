use hex;
use sha2::{Digest, Sha512};

pub fn hash_password(password: &String) -> String {
    let mut hasher = Sha512::new();
    hasher.update(password);
    let result = hasher.finalize();
    hex::encode(result)
}
