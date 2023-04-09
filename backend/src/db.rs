use anyhow::Result;
use diesel::prelude::*;

pub fn establish_connection() -> Result<SqliteConnection> {
    dotenvy::dotenv()?;
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let conn = SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url));
    Ok(conn)
}
