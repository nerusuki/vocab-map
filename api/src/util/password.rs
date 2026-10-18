use argon2::{
    Argon2, PasswordHasher, PasswordVerifier,
    password_hash::{SaltString, rand_core::OsRng},
    PasswordHash,
};

pub fn hash_password(password: &str) -> Result<String, &'static str> {
    let salt = SaltString::generate(OsRng);
    let argon2 = Argon2::default();
    let hash = argon2
        .hash_password(password.as_bytes(), &salt)
        .map_err(|_| "Could not hash password")?
        .to_string();
    Ok(hash)
}

pub fn verify_password(password: &str, hash: &str) -> bool {
    let argon2 = Argon2::default();
    let Ok(parsed_hash) = PasswordHash::new(hash) else {
        return false;
    };
    argon2.verify_password(password.as_bytes(), &parsed_hash).is_ok()
}
