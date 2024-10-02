use diesel::mysql::MysqlConnection;
use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

pub fn establish_connection() -> MysqlConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL doit être défini dans .env");
    MysqlConnection::establish(&database_url)
        .expect(&format!("Erreur lors de la connexion à {}", database_url))
}
