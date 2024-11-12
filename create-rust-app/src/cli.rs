use structopt::StructOpt;

#[derive(StructOpt)]
pub struct Cli {
    #[structopt(long)]
    name: String,
    #[structopt(long, default_value = "bin")]
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
        let mut vec = Vec::new();
        vec.push(("name", &self.name));
        vec.push(("project_type", &self.project_type));
        vec.push(("license", &self.license));
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