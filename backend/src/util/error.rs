use actix_web::{error::ResponseError, http::StatusCode, HttpResponse};
use failure::Fail;

pub type Result<T> = std::result::Result<T, KonaError>;

#[derive(Fail, Debug)]
pub enum KonaError {
    #[fail(display = "internal error")]
    InternalError,
    #[fail(display = "bad request")]
    BadClientData,
    #[fail(display = "timeout")]
    Timeout,
    #[fail(display = "session")]
    SessionError(SessionError),
}

impl ResponseError for KonaError {
    fn error_response(&self) -> HttpResponse {
        match *self {
            KonaError::InternalError => HttpResponse::new(StatusCode::INTERNAL_SERVER_ERROR),
            KonaError::BadClientData => HttpResponse::new(StatusCode::BAD_REQUEST),
            KonaError::Timeout => HttpResponse::new(StatusCode::GATEWAY_TIMEOUT),
            KonaError::SessionError(ref e) => e.error_response(),
        }
    }
}

#[derive(Fail, Debug)]
pub enum SessionError {
    #[fail(display = "missing authorization header")]
    MissingHeader,
    #[fail(display = "incorrect token type")]
    IncorrectTokenType,
    #[fail(display = "authorization header is malformed")]
    MalformedHeader,
    #[fail(display = "token was not valid or could not be verified")]
    InvalidToken,
}

impl ResponseError for SessionError {
    fn error_response(&self) -> HttpResponse {
        match *self {
            SessionError::MissingHeader => HttpResponse::BadRequest().finish(),
            SessionError::IncorrectTokenType => HttpResponse::BadRequest().finish(),
            SessionError::MalformedHeader => HttpResponse::BadRequest().finish(),
            SessionError::InvalidToken => HttpResponse::Unauthorized().finish(),
        }
    }
}
