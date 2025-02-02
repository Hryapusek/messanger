use diesel::connection::*;
use diesel::ConnectionError;
use diesel::PgConnection;

pub fn establish_connection() -> Result<PgConnection, ConnectionError> {
    PgConnection::establish(
        &std::env::var("DATABASE_URL").expect("DATABASE_URL must be set in env file"),
    )
}
