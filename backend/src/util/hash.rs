use argonautica::{input::Password, Error, Hasher, Verifier};

pub fn hash<'a, P>(password: P) -> Result<String, Error>
where
    P: Into<Password<'a>>,
{
    Hasher::default()
        .with_secret_key("foo")
        .with_password(password)
        .hash()
}

pub fn verify<'a, P, S>(password: P, hash: S) -> Result<bool, Error>
where
    P: Into<Password<'a>>,
    S: AsRef<str>,
{
    Verifier::default()
        .with_secret_key("foo")
        .with_password(password)
        .with_hash(hash)
        .verify()
}
