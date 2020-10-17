use diesel::mysql::MysqlConnection;
use diesel::r2d2::{ConnectionManager, Pool, PoolError};
use std::env;

pub type MySqlConnectionPool = Pool<ConnectionManager<MysqlConnection>>;

fn init_pool(database_url: &str) -> Result<MySqlConnectionPool, PoolError> {
    let manager = ConnectionManager::<MysqlConnection>::new(database_url);
    Pool::builder().build(manager)
}

pub fn establish_connection() -> MySqlConnectionPool {
    let database_url = env::var("DATABASE_URL").expect("The Database URL should be set");
    init_pool(&database_url).expect(&format!("Error connecting to {}", database_url))
}