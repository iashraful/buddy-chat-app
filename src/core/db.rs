use deadpool_diesel::{Manager, Pool};
use diesel::prelude::*;
use log::info;

pub fn establish_connection(db_url: String) -> Pool<Manager<PgConnection>> {
    info!("Creating new connection pool.");

    let manager: Manager<PgConnection> = Manager::new(db_url, deadpool_diesel::Runtime::Tokio1);
    let pool: Pool<Manager<PgConnection>> = Pool::builder(manager).build().unwrap();

    info!("Connection pool created.");

    pool // Return the Connection Pool
}
