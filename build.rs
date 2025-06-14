use std::env;
use std::path::{Path, PathBuf};
use graphql_client_codegen::{
    CodegenMode,
    GraphQLClientCodegenOptions,
    generate_module_token_stream,
};

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("contributions_query.rs");
    
    let query_path: PathBuf = "src/graphql/contributions.graphql".into();
    let schema_path: PathBuf = "src/graphql/schema.graphql".into();
    
    let options = GraphQLClientCodegenOptions::new(CodegenMode::Cli);
    let token_stream = generate_module_token_stream(query_path, &schema_path, options)
        .expect("Failed to generate GraphQL module");
    
    std::fs::write(dest_path, token_stream.to_string())
        .expect("Failed to write generated module");
    
    println!("cargo:rerun-if-changed=src/graphql/contributions.graphql");
    println!("cargo:rerun-if-changed=src/graphql/schema.graphql");
} 