use aws_config::meta::region::RegionProviderChain;
use aws_sdk_ecr::types::Repository;
use aws_sdk_ecr::Client;
use aws_sdk_ecr::{config::Region, meta::PKG_VERSION, Error};

async fn describe_repositories(client: &Client) -> Result<Vec<Repository>, aws_sdk_ecr::Error> {
    let rsp = client.describe_repositories().send().await?;
    let repositories = rsp.repositories().to_vec();

    Ok(repositories)
}

async fn describe_images(
    client: &aws_sdk_ecr::Client,
    name: &str,
) -> Result<Vec<String>, aws_sdk_ecr::Error> {
    let rsp = client.list_images().repository_name(name).send().await?;
    let images = rsp.image_ids();

    let mut image_tags = Vec::new();
    for image in images {
        if let Some(tag) = image.image_tag() {
            println!("Image: {:?}", tag);
            image_tags.push(tag.to_owned());
        }
    }

    Ok(image_tags)
}

fn create_image_identifier_from_tag(tag: &str) -> aws_sdk_ecr::types::ImageIdentifier {
    aws_sdk_ecr::types::ImageIdentifier::builder()
        .image_tag(tag)
        .build()
}

async fn describe_image_findings(
    client: &Client,
    name: &str,
    tag: &str,
) -> Result<(), aws_sdk_ecr::Error> {
    let image_id = create_image_identifier_from_tag(tag);
    let rsp = client
        .describe_image_scan_findings()
        .repository_name(name)
        .image_id(image_id)
        .send()
        .await?;

    let findings = rsp.image_scan_findings();

    if let Some(finding) = findings {
        for f in finding.findings() {
            match f.severity() {
                Some(severity) => println!("Severity: {:?}", severity),
                None => println!("No severity"),
            }

            match f.name() {
                Some(name) => println!("Name: {:?}", name),
                None => println!("No name"),
            }

            match f.description() {
                Some(description) => println!("Description: {:?}", description),
                None => println!("No description"),
            }
        }
    }

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let _region = "eu-west-1";
    let region_provider = RegionProviderChain::first_try(Region::new("eu-west-1"))
        .or_default_provider()
        .or_else(Region::new("eu-west-1"));

    let shared_config = aws_config::from_env().region(region_provider).load().await;
    let client = aws_sdk_ecr::Client::new(&shared_config);

    let repos = describe_repositories(&client).await?;

    for repo in repos {
        let images = describe_images(&client, repo.repository_name().unwrap()).await?;
        println!("Repo: {}", repo.repository_name().unwrap_or_default());
        for image in images {
            describe_image_findings(&client, repo.repository_name().unwrap(), &image).await?;
        }
    }

    Ok(())
}
