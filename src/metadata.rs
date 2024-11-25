use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
struct MetaData {
    packages: Vec<Package>,
}

#[derive(Debug, Deserialize, Serialize)]
struct Package {
    name: String,
    version: String,
    id: String,
    license: Option<String>,
    license_file: Option<String>,
    description: Option<String>,
    source: Option<String>,
    dependencies: Vec<Dependency>,
    targets: Vec<Target>,
}

#[derive(Debug, Deserialize, Serialize)]
struct Dependency {
    name: String,
    source: Option<String>,
    req: String,
    kind: Option<String>, /// "dev" "build" "null"
    rename: Option<String>,
    optional: bool,
    uses_default_features: bool,
    features: Vec<String>,
    target: Option<String>,
    path: Option<String>,
    registry: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
struct Target {
    kind: Vec<String>, /// "bin" "lib" "bench" "example" "test" "custom-build"
    crate_types: Vec<String>,
    name: String,
    src_path: String,
    edition: String,
    required_features: Vec<String>,
}