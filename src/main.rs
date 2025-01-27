#![feature(duration_constructors)]

use crate::api::tractive::TractiveApi;
use crate::env::EnvConfig;
use log::log;
use tokio::time::{Duration, sleep};

mod api;
mod env;

#[tokio::main]
async fn main() {
    env_logger::init();

    let env = env::load_env();
    log::trace!("Loaded environment variables: {:?}", env);

    let mut tractive = TractiveApi::connect(&env.tractive_email, &env.tractive_password).await;

    // Init dawarich

    loop {
        sync(&env, &mut tractive).await;
        sleep(Duration::from_hours(1)).await;
    }
}

async fn sync(env: &EnvConfig, tractive: &mut TractiveApi) {
    log::info!("Syncing data");

    tractive.check_auth().await;

    for t in env.tractive_tracker_ids.iter() {
        let to = chrono::Local::now();
        let from = to - chrono::Duration::days(1) + chrono::Duration::minutes(1); // A little wiggle room to not make the API's checks upset

        tractive.get_positions(t, from, to).await;

    }

    // Get tractive objects
    // Get dawarich objects
    // Deduplicate
    // Send to dawarich
}
