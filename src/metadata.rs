use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;


#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct MetaData {
    pub packages: Option<Vec<Package>>,
    pub workspace_members: Option<Vec<String>>,
    pub resolve: Option<Resolve>,
    pub target_directory: String,
    pub version: i32,
    pub workspace_root: String,
    pub metadata: Value,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Package {
    pub name: String,
    pub version: String,
    pub id: String,
    pub license: Option<String>,
    pub license_file: Option<String>,
    pub description: Option<String>,
    pub source: Option<String>,
    pub dependencies: Option<Vec<Dependency>>,
    pub targets: Option<Vec<Target>>,
    pub features: BTreeMap<String, Option<Vec<String>>>,
    pub manifest_path: String,
    pub metadata: Value,
    pub publish: Option<Vec<String>>,
    pub authors: Option<Vec<String>>,
    pub categories: Option<Vec<String>>,
    pub default_run: Option<String>,
    pub rust_version: Option<String>,
    pub keywords: Option<Vec<String>>,
    pub readme: Option<String>,
    pub repository: Option<String>,
    pub homepage: Option<String>,
    pub documentation: Option<String>,
    pub edition: String,
    pub links: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Dependency {
    pub name: String,
    pub source: Option<String>,
    pub req: String,
    pub kind: Option<String>, /// "dev" "build" "null"
    pub rename: Option<String>,
    pub optional: bool,
    pub uses_default_features: bool,
    pub features: Option<Vec<String>>,
    pub target: Option<String>,
    pub path: Option<String>,
    pub registry: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Target {
    pub kind: Option<Vec<String>>, /// "bin" "lib" "bench" "example" "test" "custom-build"
    pub crate_types: Option<Vec<String>>,
    pub name: String,
    pub src_path: String,
    pub edition: String,
    #[serde(rename = "required-features")]
    pub required_features: Option<Vec<String>>,
    pub doc: bool,
    pub doctest: bool,
    pub test: bool,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Resolve {
    pub nodes: Option<Vec<Node>>,
    pub root: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Node{
    pub id: String,
    pub dependencies: Option<Vec<String>>,
    pub deps: Option<Vec<Dep>>,
    pub features: Option<Vec<String>>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Dep {
    pub name: String,
    pub pkg: String,
    pub dep_kinds: Option<Vec<DepKind>>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct DepKind {
    pub kind: Option<String>,
    pub target: Option<String>,
}