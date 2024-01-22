use aws_sdk_ecr as ecr;

#[::tokio::main]
async fn main() -> Result<(), ecr::Error> {
    let config = aws_config::load_from_env().await;
    let client = aws_sdk_ecr::Client::new(&config);

    // ... make some calls with the client

    Ok(())
}

async fn list_findings() -> Result<(), Box<dyn Error>> {
    let region_provider = Region::new("eu-west-1"); // replace with your region
    let shared_config = aws_config::from_env().region(region_provider).load().await;
    let client = Client::new(&shared_config);

    // Example: Listing repositories (you might have specific repository names)
    let resp = client.describe_repositories().send().await?;
    for repo in resp.repositories().unwrap_or_default() {
        let repo_name = repo.repository_name.as_deref().unwrap_or_default();

        // Here you would typically list images in the repository
        // and then describe image scan findings
        // This is a placeholder for the actual calls you need to make
        println!("Repository: {}", repo_name);
    }

    Ok(())
}
