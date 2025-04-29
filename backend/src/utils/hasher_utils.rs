use argon2::{
    password_hash::{rand_core::OsRng, Error, PasswordHasher, SaltString},
    Argon2, PasswordHash, PasswordVerifier,
};

pub fn argon2_enc(password: &str) -> Result<String, Error> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    Ok(argon2
        .hash_password(password.as_bytes(), &salt)?
        .to_string())
}

pub fn argon2_verify(hash: &str, password: &str) -> Result<bool, Error> {
    let hash = PasswordHash::new(hash)?;

    match Argon2::default().verify_password(password.as_bytes(), &hash) {
        Ok(_) => return Ok(true),               
        Err(Error::Password) => return Ok(false), 
        Err(e) => return Err(e),                 
    }
}
