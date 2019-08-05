use actix_web::{dev::Payload, FromRequest, HttpMessage, HttpRequest};
use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, errors::ErrorKind as JwtError, Header, TokenData, Validation};
use redis::Commands;
use serde_derive::{Deserialize, Serialize};
use uuid::Uuid;

use crate::controllers::user::LoginForm;
use crate::util::{
    error::{KonaError, SessionError},
    CachePool,
};

pub const KEY: &str = "foo";

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginClaims {
    jti: String,
    sub: String,
    iss: String,
    iat: i64,
    exp: i64,
    uid: String,
}

impl LoginClaims {
    pub fn jti(&self) -> &str {
        &self.jti
    }
}

pub fn issue(key: &str, user_id: String) -> (String, String) {
    let issued = Utc::now();
    let expiration = issued + Duration::hours(1);

    let id = Uuid::new_v4().to_string();
    let claims = LoginClaims {
        jti: id.clone(),
        sub: "konabb-session".to_string(),
        iss: "konabb".to_string(),
        iat: issued.timestamp(),
        exp: expiration.timestamp(),
        uid: user_id
    };

    let token = encode(&Header::default(), &claims, key.as_ref()).unwrap();

    (id, token)
}

pub fn validate(jwt: String, key: &str) -> LoginClaims {
    decode(&jwt, key.as_ref(), &Validation::default())
        .unwrap()
        .claims
}

#[derive(Debug, PartialEq)]
pub enum JwtSession {
    Missing,
    Expired,
    LoggedIn(String),
}

impl JwtSession {
    pub fn has_expired(&self) -> bool {
        *self == JwtSession::Expired
    }

    pub fn is_valid(&self) -> bool {
        *self != JwtSession::Expired && *self != JwtSession::Missing
    }

    pub fn user_id(&self) -> Option<&str> {
        if let JwtSession::LoggedIn(ref user_id) = self {
            return Some(user_id);
        }

        return None;
    }
}

impl FromRequest for JwtSession {
    type Error = KonaError;
    type Future = Result<JwtSession, KonaError>;
    type Config = ();

    #[inline]
    fn from_request(req: &HttpRequest, payload: &mut Payload) -> Self::Future {
        let session_cookie = if let Ok(cookies) = req.cookies() {
            cookies
                .iter()
                .find(|cookie| cookie.name() == "konabb-session")
                .map(|cookie| cookie.clone())
        } else {
            None
        };

        let session_cookie = if session_cookie.is_some() {
            session_cookie.unwrap()
        } else {
            return Ok(JwtSession::Missing);
        };

        let session = match decode::<LoginClaims>(
            session_cookie.value(),
            KEY.as_ref(),
            &Validation::default(),
        ) {
            Ok(TokenData { header, claims }) => {
                let session_id = claims.jti();

                let cache_pool = req.app_data::<CachePool>().unwrap();
                let cache_conn = &mut cache_pool.get().unwrap();

                if let Ok(user_id) = cache_conn.get(session_id) {
                    JwtSession::LoggedIn(user_id)
                } else {
                    JwtSession::Expired
                }
            }
            Err(e) => match e.kind() {
                JwtError::ExpiredSignature => JwtSession::Expired,
                _ => return Err(KonaError::SessionError(SessionError::InvalidToken)),
            },
        };

        Ok(session)
    }
}
