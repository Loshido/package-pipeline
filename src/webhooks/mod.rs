use axum::{Json, http::StatusCode};
use types::PackageWebhook;
use crate::kube;

mod types;
pub mod auth;

pub async fn handle(Json(payload): Json<PackageWebhook>) -> StatusCode {
    if payload.action != "updated" {
        return StatusCode::EXPECTATION_FAILED;
    }

    println!("📦 Package webhook received, initiating deployments restart for {}", payload.package.name);

    match kube::restart_from_package(&payload.package.name).await {
        Ok(deployments) => {
            if deployments.len() == 0 {
                println!("No deployment found for {}.", payload.package.name);
            } else {
                println!("{} has been restarted", deployments.join(", "));
            }

            StatusCode::OK
        },
        Err(e) => {
            eprintln!("{}", e);
            StatusCode::BAD_REQUEST
        }
    }
}
