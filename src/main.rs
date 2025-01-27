#![feature(duration_constructors)]

use geojson::Feature;
use crate::api::tractive::TractiveApi;
use crate::env::EnvConfig;
use log::{debug, info, log};
use tokio::time::{Duration, sleep};
use crate::api::dawarich::{BulkPoints, DawarichApi};

mod api;
mod env;

#[tokio::main]
async fn main() {
    env_logger::init();

    let env = env::load_env();
    log::trace!("Loaded environment variables: {:?}", env);

    let mut tractive = TractiveApi::connect(&env.tractive_email, &env.tractive_password).await;
    let dawarich = DawarichApi::new(&env.dawarich_host, &env.dawarich_api_key);


    // loop {
        sync(&mut tractive, dawarich).await;
    //     sleep(Duration::from_hours(1)).await;
    // }
}

async fn sync(tractive: &mut TractiveApi, dawarich: DawarichApi) {
    info!("Running hourly sync");

    tractive.check_auth().await;

    let trackers = tractive.get_trackers().await;

    info!("Syncing {} trackers", trackers.len());

    for t in trackers {
        let to = chrono::Local::now();
        let from = to - chrono::Duration::days(1) - chrono::Duration::minutes(1); // A little wiggle room to not make the API's checks upset

        let positions = tractive.get_positions(t, from, to).await;

        let vec: Vec<Feature> = positions.iter().map(Into::into).collect();
        let bulk_points: BulkPoints = vec.into();
        dawarich.insert_bulk_points(bulk_points).await;
    }
}