use aws_config::meta::region::RegionProviderChain;
use aws_sdk_ecr::{config::Region, meta::PKG_VERSION, Error};
use aws_sdk_ecr::types::Repository;
use aws_sdk_ecr::Client;

async fn describe_repositories(client: &Client) -> Result<Vec::<Repository>, aws_sdk_ecr::Error> {
    let rsp = client.describe_repositories().send().await?;
    let repositories = rsp.repositories().to_vec();

    Ok(repositories)
}

async fn describe_images(client: &Client, name: &str) -> Result<(), aws_sdk_ecr::Error> {
   let rsp = client.list_images().repository_name(name).send().await?;
   let images = rsp.image_ids();

   for image in images {
       println!("Image: {:?}", image.image_tag())
   }

   Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let region = "eu-west-1";
    let region_provider = RegionProviderChain::first_try(Region::new("eu-west-1"))
        .or_default_provider()
        .or_else(Region::new("eu-west-1"));

    println!();
    println!("ECR client version: {}", PKG_VERSION);
    println!(
        "Region:             {}",
        region_provider.region().await.unwrap().as_ref()
    );
    println!();

    let shared_config = aws_config::from_env().region(region_provider).load().await;
    let client = aws_sdk_ecr::Client::new(&shared_config);

    let repos = describe_repositories(&client).await?;

    for repo in repos {
        describe_images(&client, repo.repository_name().unwrap()).await?;
        println!("Repo: {}", repo.repository_name().unwrap_or_default());
    }

    Ok(())
}
