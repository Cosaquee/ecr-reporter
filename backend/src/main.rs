use aws_config::meta::region::RegionProviderChain;
use aws_sdk_ecr::{config::Region, meta::PKG_VERSION, Error};
use aws_sdk_ecr::types::Repository;

async fn show_images(
    client: &aws_sdk_ecr::Client,
    repository: &str,
) -> Result<(), aws_sdk_ecr::Error> {
    let rsp = client
        .list_images()
        .repository_name(repository)
        .send()
        .await?;

    let images = rsp.image_ids();

    println!("found {} images", images.len());

    for image in images {
        println!(
            "image: {}:{}",
            image.image_tag().unwrap(),
            image.image_digest().unwrap()
        );
    }

    Ok(())
}

async fn describe_repositories(client: &aws_sdk_ecr::Client) -> Result<Vec::<Repository>, aws_sdk_ecr::Error> {
    let rsp = client.describe_repositories().send().await?;
    let repositories = rsp.repositories().to_vec();

    Ok(repositories)
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
        println!("Repo: {}", repo.repository_name().unwrap_or_default());
    }

    Ok(())
}
