use serde::{Deserialize, Serialize};
use std::collections::HashMap;

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
    pub config: Config,
    pub tags: Vec<String>,
    pub description: Option<String>,
    pub columns: HashMap<String, Column>,
    pub meta: HashMap<String, serde_json::Value>,
    pub group: Option<String>,
    pub docs: Docs,
    pub patch_path: Option<String>,
    pub build_path: Option<String>,
    pub deferred: bool,
    pub unrendered_config: UnrenderedConfig,
    pub created_at: f64,
    pub relation_name: Option<String>,
    pub raw_code: String,
    pub language: String,
    pub refs: Vec<Ref>,
    pub sources: Vec<serde_json::Value>,
    pub metrics: Vec<serde_json::Value>,
    pub depends_on: DependsOn,
    pub compiled_path: String,
    pub compiled: bool,
    pub compiled_code: String,
    pub extra_ctes_injected: bool,
    pub extra_ctes: Vec<serde_json::Value>,
    pub contract: Contract,
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
    pub enabled: bool,
    pub alias: Option<String>,
    pub schema: Option<String>,
    pub database: Option<String>,
    pub tags: Vec<String>,
    pub meta: HashMap<String, serde_json::Value>,
    pub group: Option<String>,
    pub materialized: Option<String>,
    pub incremental_strategy: Option<String>,
    pub persist_docs: Option<HashMap<String, serde_json::Value>>,
    pub post_hook: Option<Vec<String>>,
    pub pre_hook: Option<Vec<String>>,
    pub quoting: Option<HashMap<String, serde_json::Value>>,
    pub column_types: Option<HashMap<String, serde_json::Value>>,
    pub full_refresh: Option<bool>,
    pub unique_key: Option<String>,
    pub on_schema_change: Option<String>,
    pub on_configuration_change: Option<String>,
    pub grants: Option<HashMap<String, serde_json::Value>>,
    pub packages: Option<Vec<serde_json::Value>>,
    pub docs: Option<Docs>,
    pub contract: Option<Contract>,
    pub access: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Docs {
    pub show: bool,
    pub node_color: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UnrenderedConfig {
    pub materialized: Option<String>,
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
    pub macros: Vec<String>,
    pub nodes: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Contract {
    pub enforced: bool,
    pub alias_types: bool,
    pub checksum: Option<serde_json::Value>,
}

// Add struct for run_results
use serde::{Serialize, Deserialize};
use std::collections::HashMap;

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


fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    // open sample/target/catalog.json and populate the manifest object
    let manifest_path = "sample/target/manifest.json";
    let file = std::fs::File::open(manifest_path)?;
    let reader = std::io::BufReader::new(file);
    let manifest: Manifest = serde_json::from_reader(reader)?;

    // Return the SQL for a model by passing the model name
    let model_name = "my_second_dbt_model"; // Change this to your actual model name
    let full_model_name = build_model_key("sample", model_name);
    if let Some(model) = manifest.nodes.get(&full_model_name) {
        println!("Model SQL: {}", model.raw_code);
        println!("Compiled SQL: {}", model.compiled_code);
    } else {
        println!("Model {} not found under key {}", model_name, full_model_name);
    }

    Ok(())
}

fn build_model_key(prefix: &str, model_name: &str) -> String {
    format!("model.{}.{}", prefix, model_name) // Adjust format if your model keys differ
}