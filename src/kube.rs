use k8s_openapi::api::apps::v1::Deployment;
use kube::{Api, Client, api::ListParams, runtime::reflector::Lookup};
use anyhow::{Result, Context};

async fn get_client() -> Result<Client> {
    let client = Client::try_default()
        .await
        .context("Unable to access in-cluster config")?;

    Ok(client)
}

pub async fn restart_from_package(package: &str) -> Result<Vec<String>> {
    let client = self::get_client().await?;

    let api: Api<Deployment> = Api::namespaced(client, "default");

    let label = format!("nogata.package-pipeline={}", package);
    let lp = ListParams::default().labels(&label);
    let mut deployments = Vec::new();

    for deployment in api.list(&lp).await? {
        if let Some(name) = deployment.name() {
            api
                .restart(&name)
                .await
                .with_context(|| format!("Unable to restart deployment {}", name))?;

            deployments.push(name.to_string())
        }
    }

    Ok(deployments)
}