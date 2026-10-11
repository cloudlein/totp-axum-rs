use std::env;

#[derive(Clone)]
pub struct Config{
    pub database_url: String,
    pub postgres_user: String,
    pub postgres_password: String,
    pub postgres_db: String,
    pub postgres_port: u16
}

impl Config{
    pub fn init() -> Self {
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set on env");
        let postgres_user = env::var("POSTGRES_USER").expect("POSTGRES_USER must be set on .env");
        let postgres_password = env::var("POSTGRES_PASSWORD").expect("POSTGRES_PASSWORD must be set on .env");
        let postgres_db = env::var("POSTGRES_DB").expect("POSTGRES_DB must be set on .env");
        let postgres_port = env::var("POSTGRES_PORT")
            .expect("POSTGRES_PORT must be set on .env")
            .parse::<u16>()
            .expect("POSTGRES_PORT must be a valid port number");
        Self {database_url, postgres_user, postgres_password, postgres_db, postgres_port}
    }
}