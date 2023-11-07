use std::str::FromStr;

use log::info;
use sqlx::migrate::MigrateDatabase;
use sqlx::sqlite::SqliteConnectOptions;
use sqlx::Transaction;
use sqlx::{migrate::Migrator, Pool, Sqlite, SqlitePool};

#[derive(Debug, Clone)]
pub struct Database {
    pub pool: Pool<Sqlite>,
}

impl Database {
    pub async fn new(database_url: &str, migrator: &Migrator) -> Result<Self, sqlx::Error> {
        if !Sqlite::database_exists(&database_url).await.unwrap_or(false) {
            info!("No Database file found... Creating Database {}", &database_url);
            match Sqlite::create_database(&database_url).await {
                Ok(_) => info!("Database file created"),
                Err(error) => panic!("error: {}", error),
            }
        } else {
            info!("Database file {} already exists", &database_url);
        }
        let pool_options = SqliteConnectOptions::from_str(&database_url).expect("failed to configure options for DB connection");
        let db_pool = SqlitePool::connect_with(pool_options).await?;
        migrator.run(&db_pool).await.expect("Failed to apply migrations to DB");

        Result::Ok(Database { pool: db_pool })
    }

    pub async fn new_tx(&self) -> Result<Transaction<Sqlite>, sqlx::Error> {
        self.pool.begin().await
    }
}
