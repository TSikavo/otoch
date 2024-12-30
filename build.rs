use std::{env, fs};
use std::path::Path;

fn main() {
    // Read Cargo.toml
    let cargo_toml_path = Path::new(&env::var("CARGO_MANIFEST_DIR").unwrap()).join("Cargo.toml");
    let cargo_toml_content = fs::read_to_string(&cargo_toml_path).expect("Failed to read Cargo.toml");
    let cargo_toml: toml::Value = toml::from_str(&cargo_toml_content).expect("Cargo.toml is not valid");

    // Getting [package.metadata]
    if let Some(package_metadata) = cargo_toml.get("package").and_then(|pkg| pkg.get("metadata")) {
        let app_id = package_metadata.get("app_id").expect("The [package.metadata] section in Cargo.toml does not contain `app_id`]").as_str().expect("'app_id' in [package.metadata] in Cargo.toml must be a string");
        println!("cargo:rustc-env=APP_ID={}", app_id);
    } else {
        panic!("Cargo.toml does not have a [package.metadata] section");
    }
}