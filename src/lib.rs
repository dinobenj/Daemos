#[derive(Debug, Clone)]
pub struct Container {
    pub image: String,
    pub id: String,
}

#[derive(Debug, Clone)]
pub struct Daimon {
    pub image: String,
    pub count: usize,
    pub containers: Vec<Container>,
}

impl Daimon {
    pub fn new(image: String, count: usize) -> Self {
        Daimon {
            image,
            count,
            containers: Vec::new(),
        }
    }

    pub fn add_container(&mut self, id: String) {
        self.containers.push(Container {
            image: self.image.clone(),
            id,
        });
    }
}


pub fn parse_container_group(s: &str) -> Result<Daimon, String> {
    let parts: Vec<&str> = s.split(':').collect();
    if parts.len() != 2 {
        return Err("Expected format image-name:N".into());
    }

    let image = parts[0].to_string();
    let count = parts[1]
        .parse::<usize>()
        .map_err(|_| "Second part must be a number")?;

    Ok(Daimon::new(image, count))
}