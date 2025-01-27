#![feature(duration_constructors)]

use crate::api::tractive::TractiveApi;
use crate::env::EnvConfig;
use log::{info, log};
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

    // loop {
        sync(&mut tractive).await;
    //     sleep(Duration::from_hours(1)).await;
    // }
}

async fn sync(tractive: &mut TractiveApi) {
    info!("Running hourly sync");

    tractive.check_auth().await;

    let trackers = tractive.get_trackers().await;

    info!("Syncing {} trackers", trackers.len());

    for t in trackers {
        let to = chrono::Local::now();
        let from = to - chrono::Duration::days(1) - chrono::Duration::minutes(1); // A little wiggle room to not make the API's checks upset

        tractive.get_positions(t, from, to).await;

    }

    // Get tractive objects
    // Get dawarich objects
    // Deduplicate
    // Send to dawarich
}
