use dotenv::dotenv;
use std::env;
use sqlx::{PgPool, Row};

#[tokio::main]
async fn main() {
    // Load environment variables from the .env file
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    println!("Attempting to connect to the database...");

    // Connect to the PostgreSQL database
    let pool = PgPool::connect(&database_url).await;

    match pool {
        Ok(pool) => {
            println!("✅ Successfully connected to the database!");

            // Perform a simple query to check the database version
            let row = sqlx::query("SELECT version()")
                .fetch_one(&pool)
                .await;

            match row {
                Ok(row) => {
                    let version: String = row.get(0);
                    println!("PostgreSQL version: {}", version);
                }
                Err(err) => {
                    eprintln!("⚠️ Failed to fetch PostgreSQL version: {}", err);
                }
            }
        }
        Err(err) => {
            eprintln!("❌ Failed to connect to the database: {}", err);
        }
    }
}
