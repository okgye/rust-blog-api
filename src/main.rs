use sqlx::PgPool;

#[tokio::main]
async fn main() {
    // Replace with your actual database URL
    let database_url = "postgres://postgres:your_password@172.18.0.2:5432/your_database";

    println!("⏳ Attempting to connect to the database...");

    // Create a connection pool
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
