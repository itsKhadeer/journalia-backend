pub mod auth;
pub mod error;
pub mod user;
pub mod util;

pub type PgPool = diesel::r2d2::Pool<diesel::r2d2::ConnectionManager<diesel::PgConnection>>;
pub type RedisPool = r2d2::Pool<redis::Client>;
pub type RedisConn = r2d2::PooledConnection<redis::Client>;
