use tokio::{signal, time::{sleep, Duration}};
use app::{config::get_db_connection, router::init_router, state::AppState};

type GenericResult<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    if let Err(e) = run().await {
        println!("Application failed to start {:?}", e);
        sleep(Duration::from_millis(100)).await;
        std::process::exit(1);
    }
}


async fn run() -> GenericResult<()> {
    println!("Starting application...");
    
    let db = get_db_connection().await?;
    // let redis_pool = get_redis_pool().await?;
    let state = AppState { db };

    let app = init_router(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    println!("Server listening on 0.0.0.0:3000");
    
    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await?;

    Ok(())
}

async fn shutdown_signal() {
    let _ = signal::ctrl_c().await;
    println!("shutdown signal received");
}
