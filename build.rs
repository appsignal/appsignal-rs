use std::env;
use std::path::Path;

#[path = "src/download.rs"]
mod download;

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let out_path = Path::new(&out_dir);

    // Determine system architecture
    let target = env::var("TARGET").unwrap();

    download::download_agent_archive(
        target.as_str(),
        &out_path
    );

    // Let Cargo know it needs to link the library
    println!("cargo:rustc-link-search=native={}", &out_dir);
    println!("cargo:rustc-link-lib=static=appsignal");
}
