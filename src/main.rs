use std::clone;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "daemos")]
struct Args {
    #[arg(long = "container", value_parser = parse_container_group)]
    containers: Vec<ContainerGroup>,

    #[arg(long)]
    balancer: String,
}

#[derive(Debug, Clone)]
struct ContainerGroup {
    image: String,
    count: usize,
}
impl ContainerGroup {
    fn new(image: String, count: usize) -> Self {
        ContainerGroup { image, count }
    }

    fn clone(&self) -> Self {
        ContainerGroup {
            image: self.image.clone(),
            count: self.count,
        }
    }
}

fn parse_container_group(s: &str) -> Result<ContainerGroup, String> {
    let parts: Vec<&str> = s.split(':').collect();
    if parts.len() != 2 {
        return Err("Expected format image-name:N".into());
    }

    let image = parts[0].to_string();
    let count = parts[1]
        .parse::<usize>()
        .map_err(|_| "Second part must be a number")?;

    Ok(ContainerGroup { image, count })
}

async fn start_container(image: String, count: usize) {
    println!("Starting container: {} x{}", image, count);
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    println!("Containers: {:?}", args.containers);
    println!("Balancer: {}", args.balancer);
    for container in args.containers {
        start_container(container.image, container.count).await;
    }
    let docker = docker_api::Docker::new("tcp://127.0.0.1:80").unwrap();
    match docker.images().list(&Default::default()).await {
        Ok(images) => {
            for image in images {
                println!("{0:?}", image.repo_tags);
            }
        },
        Err(e) => eprintln!("Something bad happened! {e}"),
    }
}