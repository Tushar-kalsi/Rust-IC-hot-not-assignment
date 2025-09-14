use std::env;
use std::fs;
use std::path::Path;

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("built.rs");

    // Generate build info
    let build_info = format!(
        r#"
pub const BUILD_TIME: &str = "{}";
pub const GIT_HASH: &str = "{}";
"#,
        chrono::Utc::now().format("%Y-%m-%d %H:%M:%S UTC"),
        env::var("GIT_HASH").unwrap_or_else(|_| "unknown".to_string())
    );

    fs::write(dest_path, build_info).unwrap();

    println!("cargo:rerun-if-env-changed=GIT_HASH");
}