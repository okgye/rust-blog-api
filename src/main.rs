use sqlx::PgPool;
use dotenvy::dotenv;
use std::env;

#[tokio::main]
async fn main() {
    if dotenv().is_err() {
        println!("⚠️  .env file not found or not loaded.");
    }

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    println!("⏳ Attempting to connect to the database...");

    match PgPool::connect(&database_url).await {
        Ok(pool) => {
            println!("✅ Successfully connected to the database!");

            // Optional: Run a simple query to verify further
            match sqlx::query("SELECT 1").execute(&pool).await {
                Ok(_) => println!("✅ Successfully executed test query!"),
                Err(err) => eprintln!("❌ Failed to execute test query: {:?}", err),
            }
        }
        Err(err) => eprintln!("❌ Failed to connect to the database: {:?}", err),
    }
}
