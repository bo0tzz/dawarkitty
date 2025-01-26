use clokwerk::{AsyncScheduler, TimeUnits};
use log::log;
use crate::api::tractive::TractiveApi;

mod api;
mod env;

#[tokio::main]
async fn main() {
    env_logger::init();

    let env = env::load_env();
    log::trace!("Loaded environment variables: {:?}", env);

    let tractive = TractiveApi::connect(&env.tractive_email, &env.tractive_password).await;
    let expiry = tractive.auth.unwrap().expires_at;
    log::debug!("Tractive initialized, token expires at: {}", expiry);

    // Init dawarich

    let mut scheduler = AsyncScheduler::new();
    scheduler.every(1.hour()).run(|| async { sync().await });

    loop {
        scheduler.run_pending().await;
        tokio::time::sleep(std::time::Duration::from_secs(60)).await;
    }
}



async fn sync() {
    log::info!("Syncing data");

    // Get tractive objects
    // Get dawarich objects
    // Deduplicate
    // Send to dawarich
}
