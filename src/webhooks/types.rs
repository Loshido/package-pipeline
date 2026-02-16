use serde::Deserialize;

#[allow(dead_code)]
#[derive(Deserialize)]
pub struct PackageWebhook {
    pub action: String,
    pub package: Package,
    pub repository: Repository,
}

#[allow(dead_code)]
#[derive(Deserialize)]
pub struct Package {
    pub name: String,
    pub package_type: String,
}

#[allow(dead_code)]
#[derive(Deserialize)]
pub struct Repository {
    pub full_name: String,
}