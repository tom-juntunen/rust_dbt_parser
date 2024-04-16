use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::env;
use std::time::Instant;

// // Structs for catalog.json
// #[derive(Debug, Deserialize, Serialize)]
// pub struct Catalog {
//     pub metadata: Metadata,
//     pub nodes: HashMap<String, Node>,
//     pub sources: HashMap<String, Source>,
//     pub errors: Option<serde_json::Value>,
// }

// #[derive(Debug, Deserialize, Serialize)]
// pub struct Metadata {
//     pub dbt_schema_version: String,
//     pub dbt_version: String,
//     pub generated_at: String,
//     pub invocation_id: String,
//     pub env: HashMap<String, serde_json::Value>,
// }

// #[derive(Debug, Deserialize, Serialize)]
// pub struct Node {
//     // Add fields as needed to represent data in the "nodes" object
// }

// #[derive(Debug, Deserialize, Serialize)]
// pub struct Source {
//     // Add fields as needed to represent data in the "sources" object
// }

// // Structs for graph summary
// #[derive(Debug, Deserialize, Serialize)]
// pub struct GraphSummary {
//     pub _invocation_id: String,
//     pub linked: HashMap<String, LinkedNode>,
// }

// #[derive(Debug, Deserialize, Serialize)]
// pub struct LinkedNode {
//     pub name: String,
//     pub r#type: String,
//     pub succ: Vec<usize>,
// }

// Structs for manifest.json
#[derive(Debug, Deserialize, Serialize)]
pub struct Manifest {
    pub nodes: HashMap<String, ManifestNode>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ManifestNode {
    pub database: String,
    pub schema: String,
    pub name: String,
    pub resource_type: String,
    pub package_name: String,
    pub path: String,
    pub original_file_path: String,
    pub unique_id: String,
    pub fqn: Vec<String>,
    pub alias: String,
    pub checksum: Checksum,
    pub config: Option<Config>,
    pub tags: Vec<String>,
    pub description: Option<String>,
    pub columns: HashMap<String, Column>,
    pub meta: HashMap<String, serde_json::Value>,
    pub group: Option<String>,
    pub docs: Docs,
    pub patch_path: Option<String>,
    pub build_path: Option<String>,
    pub deferred: bool,
    // pub unrendered_config: Option<UnrenderedConfig>,
    pub created_at: f64,
    pub relation_name: Option<String>,
    pub raw_code: String,
    pub language: Option<String>,
    pub refs: Option<Vec<Ref>>,
    pub sources: Option<Vec<serde_json::Value>>,
    pub metrics: Option<Vec<serde_json::Value>>,
    pub depends_on: DependsOn,
    pub compiled_path: Option<String>,
    pub compiled: Option<bool>,
    pub compiled_code: Option<String>,
    pub extra_ctes_injected: Option<bool>,
    pub extra_ctes: Option<Vec<serde_json::Value>>,
    pub contract: Option<Contract>,
    pub access: Option<String>,
    pub constraints: Option<Vec<serde_json::Value>>,
    pub version: Option<String>,
    pub latest_version: Option<String>,
    pub deprecation_date: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Checksum {
    pub name: String,
    pub checksum: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    // pub enabled: bool,
    // pub alias: Option<String>,
    // pub schema: Option<String>,
    // pub database: Option<String>,
    // pub tags: Option<Vec<String>>,
    // pub meta: Option<HashMap<String, serde_json::Value>>,
    // pub group: Option<String>,
    // pub materialized: Option<String>,
    // pub incremental_strategy: Option<String>,
    // pub persist_docs: Option<HashMap<String, serde_json::Value>>,
    // pub post_hook: Option<Vec<Hook>>,
    // pub pre_hook: Option<Vec<Hook>>,
    // pub quoting: Option<HashMap<String, serde_json::Value>>,
    // pub column_types: Option<HashMap<String, serde_json::Value>>,
    // pub full_refresh: Option<bool>,
    pub unique_key: Option<UniqueKey>,
    // pub on_schema_change: Option<String>,
    // pub on_configuration_change: Option<String>,
    // pub grants: Option<HashMap<String, serde_json::Value>>,
    // pub packages: Option<Vec<serde_json::Value>>,
    // pub docs: Option<Docs>,
    // pub contract: Option<Contract>,
    // pub access: Option<String>,
    // pub copy_grants: Option<String>,
    // pub indexes: Option<Vec<Index>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Hook {
    sql: Option<String>,
    transaction: Option<bool>,
    index: Option<u32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Index {
    pub columns: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UniqueKey {
    Multiple(Vec<String>),
    Single(String),
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Docs {
    pub show: bool,
    pub node_color: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UnrenderedConfig {
    pub copy_grants: Option<String>,
    pub schema: Option<String>,
    pub materialized: Option<String>,
    pub incremental_strategy: Option<String>,
    pub unique_key: Option<UniqueKey>,
    pub indexes: Option<Vec<Index>>,
    pub pre_hook: Option<String>,
    pub post_hook: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Column {
    pub name: String,
    pub description: String,
    pub meta: HashMap<String, serde_json::Value>,
    pub data_type: Option<String>,
    pub constraints: Vec<serde_json::Value>,
    pub quote: Option<String>,
    pub tags: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Ref {
    pub name: String,
    pub package: Option<serde_json::Value>,
    pub version: Option<serde_json::Value>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct DependsOn {
    pub macros: Option<Vec<String>>,
    pub nodes: Option<Vec<String>>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Contract {
    pub enforced: bool,
    pub alias_types: bool,
    pub checksum: Option<serde_json::Value>,
}

#[derive(Serialize, Deserialize, Debug)]
struct RunResults {
    metadata: Metadata,
    results: Vec<ResultNode>,
    elapsed_time: f64,
    args: Args,
}

#[derive(Serialize, Deserialize, Debug)]
struct Metadata {
    dbt_schema_version: String,
    dbt_version: String,
    generated_at: String,
    invocation_id: String,
    env: HashMap<String, serde_json::Value>,
}

#[derive(Serialize, Deserialize, Debug)]
struct ResultNode {
    status: String,
    timing: Vec<Timing>,
    thread_id: String,
    execution_time: f64,
    adapter_response: HashMap<String, serde_json::Value>,
    message: Option<String>,
    failures: Option<serde_json::Value>,  // Could be a more specific type depending on the data
    unique_id: String,
    compiled: bool,
    compiled_code: String,
    relation_name: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Timing {
    name: String,
    started_at: String,
    completed_at: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Args {
    log_file_max_bytes: usize,
    version_check: bool,
    macro_debugging: bool,
    static_: bool,
    quiet: bool,
    static_parser: bool,
    introspect: bool,
    use_colors_file: bool,
    project_dir: String,
    log_format_file: String,
    warn_error_options: WarnErrorOptions,
    which: String,
    compile: bool,
    vars: HashMap<String, serde_json::Value>,
    populate_cache: bool,
    write_json: bool,
    printer_width: usize,
    cache_selected_only: bool,
    use_colors: bool,
    exclude: Vec<String>,
    profiles_dir: String,
    show_resource_report: bool,
    log_format: String,
    invocation_command: String,
    empty_catalog: bool,
    strict_mode: bool,
    enable_legacy_logger: bool,
    defer: bool,
    log_level: String,
    partial_parse_file_diff: bool,
    log_path: String,
    indirect_selection: String,
    select: Vec<String>,
    partial_parse: bool,
    log_level_file: String,
    send_anonymous_usage_stats: bool,
    favor_state: bool,
    print: bool,
}

#[derive(Serialize, Deserialize, Debug)]
struct WarnErrorOptions {
    include: Vec<String>,
    exclude: Vec<String>,
}



fn main() -> Result<(), Box<dyn std::error::Error>> {
    let start = Instant::now();  // Start timing
    // open sample/target/catalog.json and populate the manifest object
    let manifest_path = "sample/target/manifest.json";
    let file = std::fs::File::open(manifest_path)?;
    let reader = std::io::BufReader::new(file);
    let manifest: Manifest = serde_json::from_reader(reader)?;

    // Retrieve the model name from command-line arguments
    let args: Vec<String> = env::args().collect();
    let model_name = match args.get(1) {
        Some(name) => name,
        None => {
            eprintln!("Please provide the model name as a command-line argument.");
            return Ok(());
        }
    };

    // Build the full model name using the provided model name
    let full_model_name = build_model_key("sample", model_name);
    if let Some(model) = manifest.nodes.get(&full_model_name) {
        println!("Model SQL: {}", model.raw_code);
        println!("Compiled SQL: {:?}", model.compiled_code);
    } else {
        println!("Model {} not found under key {}", model_name, full_model_name);
    }

    let duration = start.elapsed();
    println!("Time elapsed is: {:?}", duration);

    Ok(())
}


fn build_model_key(prefix: &str, model_name: &str) -> String {
    format!("model.{}.{}", prefix, model_name) // Adjust format if your model keys differ
}