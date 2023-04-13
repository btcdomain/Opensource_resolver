use rocket_sync_db_pools::database;
use diesel::MysqlConnection;

#[database("mysql_database")]
pub struct DbConn(MysqlConnection);