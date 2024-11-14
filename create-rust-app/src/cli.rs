use structopt::StructOpt;

#[derive(StructOpt)]
pub struct Cli {
    #[structopt(long)]
    name: String,
    #[structopt(long, default_value = "cli")]
    project_type: String,
    #[structopt(long, default_value = "MIT")]
    license: String,
    #[structopt(long)]
    include_ci: Option<String>,
    #[structopt(long)]
    dependencies: Option<String>,
    #[structopt(long)]
    dev_dependencies: Option<String>,
}

impl Cli {
    pub fn iter(&self) -> impl Iterator<Item = (&str, &String)> {
        let mut vec = vec![
            ("name", &self.name),
            ("project_type", &self.project_type),
            ("license", &self.license),
        ];
        if let Some(ref include_ci) = self.include_ci {
            vec.push(("include_ci", include_ci));
        }
        if let Some(ref dependencies) = self.dependencies {
            vec.push(("dependencies", dependencies));
        }
        if let Some(ref dev_dependencies) = self.dev_dependencies {
            vec.push(("dev_dependencies", dev_dependencies));
        }
        vec.into_iter()
    }

    pub fn get(&self, key: &str) -> Option<&String> {
        match key {
            "name" => Some(&self.name),
            "project_type" => Some(&self.project_type),
            "license" => Some(&self.license),
            "include_ci" => self.include_ci.as_ref(),
            "dependencies" => self.dependencies.as_ref(),
            "dev_dependencies" => self.dev_dependencies.as_ref(),
            _ => None,
        }
    }
}

pub fn parse_args() -> Cli {
    Cli::from_args()
}

pub enum ProjectType {
    Cli,
    Web,
    Desktop,
}

impl std::str::FromStr for ProjectType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "cli" => Ok(ProjectType::Cli),
            "web" => Ok(ProjectType::Web),
            "desktop" => Ok(ProjectType::Desktop),
            _ => Err(format!("Invalid project type: {}", s)),
        }
    }
}