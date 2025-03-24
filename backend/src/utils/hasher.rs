use argon2::{
    password_hash::{rand_core::OsRng, Error, PasswordHasher, SaltString},
    Argon2, PasswordHash, PasswordVerifier,
};

pub fn argon2_enc(password: &str) -> Result<String, Error> {
    // create the salt
    let salt = SaltString::generate(&mut OsRng);
    // init the default argon parameters
    let argon2 = Argon2::default();
    // hash the password and return
    Ok(argon2
        .hash_password(password.as_bytes(), &salt)?
        .to_string())
}

pub fn argon2_verify(hash: &str, password: &str) -> Result<bool, Error> {
    // parsing the phc hash into PasswordHash type
    let hash = PasswordHash::new(hash)?;

    // verifying if the password produce the same output and return it
    match Argon2::default().verify_password(password.as_bytes(), &hash) {
        Ok(_) => return Ok(true),                 // if is equal return true
        Err(Error::Password) => return Ok(false), // if is not equal return false
        Err(e) => return Err(e),                  // if it can't verify propagate the error
    }
}
