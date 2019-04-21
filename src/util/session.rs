use std::ops::{Deref, DerefMut};

use actix_web::dev::Payload;
use actix_web::{FromRequest, HttpRequest};
use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, Header, TokenData, Validation};
use serde::de::DeserializeOwned;
use serde_derive::{Deserialize, Serialize};
use uuid::Uuid;

use crate::util::error::{KonaError, SessionError};

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginClaims {
    jti: String,
    sub: String,
    iss: String,
    iat: i64,
    exp: i64,
}

impl LoginClaims {
    pub fn jti(&self) -> &str {
        &self.jti
    }
}

pub fn issue() -> (String, String) {
    let issued = Utc::now();
    let expiration = issued + Duration::hours(1);

    let id = Uuid::new_v4().to_string();
    let claims = LoginClaims {
        jti: id.clone(),
        sub: "session".to_string(),
        iss: "konabb".to_string(),
        iat: issued.timestamp(),
        exp: expiration.timestamp(),
    };

    let token = encode(&Header::default(), &claims, "foo".as_ref()).unwrap();

    (id, token)
}

pub fn validate(jwt: String) -> LoginClaims {
    decode(&jwt, "foo".as_ref(), &Validation::default())
        .unwrap()
        .claims
}

#[derive(Debug)]
pub struct JwtSession<T> {
    data: T,
}

impl<T> Deref for JwtSession<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.data
    }
}

impl<T> DerefMut for JwtSession<T> {
    fn deref_mut(&mut self) -> &mut T {
        &mut self.data
    }
}

impl<T> FromRequest for JwtSession<T>
where
    T: DeserializeOwned + 'static,
{
    type Error = KonaError;
    type Future = Result<JwtSession<T>, KonaError>;
    type Config = ();

    #[inline]
    fn from_request(req: &HttpRequest, payload: &mut Payload) -> Self::Future {
        use actix_web::http::header::{HeaderMap, HeaderValue, AUTHORIZATION};
        use actix_web::HttpMessage;

        let headers: &HeaderMap = req.headers();

        let authorization_header: &HeaderValue = headers
            .get(AUTHORIZATION)
            .ok_or_else(|| KonaError::SessionError(SessionError::MissingHeader))?;

        let mut header_value = authorization_header
            .to_str()
            .map_err(|e| KonaError::SessionError(SessionError::MalformedHeader))?
            .splitn(2, ' ');

        match header_value.next() {
            Some(ty) if ty == "Bearer" => {}
            _ => return Err(KonaError::SessionError(SessionError::IncorrectTokenType)),
        };

        let TokenData { header, claims } = header_value
            .next()
            .map(|token| decode(&token, "foo".as_ref(), &Validation::default()))
            .ok_or_else(|| KonaError::SessionError(SessionError::InvalidToken))?
            .map_err(|e| KonaError::SessionError(SessionError::InvalidToken))?;

        Ok(JwtSession { data: claims })
    }
}
