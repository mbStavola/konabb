use diesel::{MysqlConnection, r2d2::ConnectionManager as DieselConnectionManager};
use r2d2::Pool;
use r2d2_redis::RedisConnectionManager;

pub use error::{KonaError, Result};
pub use hash::{hash, verify};
pub use markdown::parse_markdown;
pub use session::{issue, JwtSession, KEY, LoginClaims, validate};

mod error;
mod hash;
mod markdown;
mod session;

pub type CachePool = Pool<RedisConnectionManager>;
pub type DbPool = Pool<DieselConnectionManager<MysqlConnection>>;
