use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;


#[derive(Debug, Deserialize, Serialize)]
pub struct MetaData {
    packages: Option<Vec<Package>>,
    workspace_members: Option<Vec<String>>,
    resolve: Option<Resolve>,
    target_directory: String,
    version: i32,
    workspace_root: String,
    metadata: Value,
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
    dependencies: Option<Vec<Dependency>>,
    targets: Option<Vec<Target>>,
    features: BTreeMap<String, Option<Vec<String>>>,
    manifest_path: String,
    metadata: Value,
    publish: Option<Vec<String>>,
    authors: Option<Vec<String>>,
    categories: Option<Vec<String>>,
    default_run: Option<String>,
    rust_version: Option<String>,
    keywords: Option<Vec<String>>,
    readme: Option<String>,
    repository: Option<String>,
    homepage: Option<String>,
    documentation: Option<String>,
    edition: String,
    links: Option<String>,
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
    features: Option<Vec<String>>,
    target: Option<String>,
    path: Option<String>,
    registry: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
struct Target {
    kind: Option<Vec<String>>, /// "bin" "lib" "bench" "example" "test" "custom-build"
    crate_types: Option<Vec<String>>,
    name: String,
    src_path: String,
    edition: String,
    #[serde(rename = "required-features")]
    required_features: Option<Vec<String>>,
    doc: bool,
    doctest: bool,
    test: bool,
}

#[derive(Debug, Deserialize, Serialize)]
struct Resolve {
    nodes: Option<Vec<Node>>,
    root: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
struct Node{
    id: String,
    dependencies: Option<Vec<String>>,
    deps: Option<Vec<Dep>>,
    features: Option<Vec<String>>,
}

#[derive(Debug, Deserialize, Serialize)]
struct Dep {
    name: String,
    pkg: String,
    dep_kinds: Option<Vec<DepKind>>,
}

#[derive(Debug, Deserialize, Serialize)]
struct DepKind {
    kind: Option<String>,
    target: Option<String>,
}