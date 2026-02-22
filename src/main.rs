use tokio;
use app::{config::get_db_connection, state::AppState};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();

    let db = get_db_connection().await?;

    let app_state = AppState { db };
    
    
    Ok(())
}
