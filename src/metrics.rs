use crate::api::tractive::TractiveApi;
use hyper::service::service_fn;
use hyper::{body::Incoming, Request, Response, StatusCode};
use hyper_util::rt::TokioIo;
use hyper_util::server::conn::auto::Builder;
use http_body_util::Full;
use hyper::body::Bytes;
use log::{error, info, debug};
use std::convert::Infallible;
use std::net::SocketAddr;
use tokio::net::TcpListener;

pub async fn start_metrics_server(port: u16, tractive_api: TractiveApi) {
    let addr = SocketAddr::from(([0, 0, 0, 0], port));

    info!("Starting Prometheus metrics server on {}", addr);

    let listener = match TcpListener::bind(addr).await {
        Ok(listener) => listener,
        Err(e) => {
            error!("Failed to bind to {}: {}", addr, e);
            return;
        }
    };

    loop {
        let (stream, _) = match listener.accept().await {
            Ok(conn) => conn,
            Err(e) => {
                error!("Failed to accept connection: {}", e);
                continue;
            }
        };

        let io = TokioIo::new(stream);
        let tractive_clone = tractive_api.clone();

        tokio::task::spawn(async move {
            let service = service_fn(move |req| {
                let tractive = tractive_clone.clone();
                handle_request(req, tractive)
            });

            if let Err(err) = Builder::new(hyper_util::rt::TokioExecutor::new())
                .serve_connection(io, service)
                .await
            {
                error!("Error serving connection: {:?}", err);
            }
        });
    }
}

async fn handle_request(
    req: Request<Incoming>,
    mut tractive_api: TractiveApi,
) -> Result<Response<Full<Bytes>>, Infallible> {
    match req.uri().path() {
        "/metrics" => {
            debug!("Handling metrics request");

            // Get all trackers
            let trackers = tractive_api.get_trackers().await;

            let mut metrics = String::new();
            metrics.push_str("# HELP tractive_battery_level Battery level percentage of Tractive devices\n");
            metrics.push_str("# TYPE tractive_battery_level gauge\n");

            // Get battery level for each tracker
            for tracker in trackers {
                match tractive_api.get_device_hw_report(&tracker._id).await {
                    Ok(report) => {
                        metrics.push_str(&format!(
                            "tractive_battery_level{{device_id=\"{}\"}} {}\n",
                            tracker._id,
                            report.battery_level
                        ));
                    }
                    Err(e) => {
                        error!("Failed to get battery level for {}: {}", tracker._id, e);
                        // Continue with other trackers even if one fails
                    }
                }
            }

            Ok(Response::builder()
                .status(StatusCode::OK)
                .header("Content-Type", "text/plain; version=0.0.4")
                .body(Full::new(Bytes::from(metrics)))
                .unwrap())
        }
        "/" => {
            let body = "Tractive Battery Metrics Server\n\nAvailable endpoints:\n/metrics - Prometheus metrics\n";
            Ok(Response::builder()
                .status(StatusCode::OK)
                .body(Full::new(Bytes::from(body)))
                .unwrap())
        }
        _ => {
            Ok(Response::builder()
                .status(StatusCode::NOT_FOUND)
                .body(Full::new(Bytes::from("404 Not Found")))
                .unwrap())
        }
    }
}
