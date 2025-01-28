use crate::api::dawarich::{BulkPoints, DawarichApi};
use crate::api::tractive::TractiveApi;
use geojson::Feature;
use log::info;
use tokio::time::{sleep, Duration};

mod api;
mod env;

#[tokio::main]
async fn main() {
    env_logger::init();

    let env = env::load_env();
    log::trace!("Loaded environment variables: {:?}", env);

    let mut tractive = TractiveApi::connect(&env.tractive_email, &env.tractive_password).await;
    let dawarich = DawarichApi::new(&env.dawarich_host, &env.dawarich_api_key);

    loop {
        sync(&mut tractive, &dawarich).await;
        sleep(Duration::from_secs(60 * 60)).await;
    }
}

async fn sync(tractive: &mut TractiveApi, dawarich: &DawarichApi) {
    info!("Running hourly sync");

    let trackers = tractive.get_trackers().await;

    info!("Syncing {} trackers", trackers.len());

    for t in trackers {
        let to = chrono::Local::now();
        let from = to - chrono::Duration::days(1) - chrono::Duration::minutes(1); // A little wiggle room to not make the API's checks upset

        let positions = tractive.get_positions(t.clone(), from, to).await;

        let vec: Vec<Feature> = positions
            .iter()
            .map(Into::into)
            .map(|mut f: Feature| {
                f.set_property("device_id", t._id);
                f
            })
            .collect();
        let bulk_points: BulkPoints = vec.into();
        dawarich.insert_bulk_points(bulk_points).await;
    }
}
