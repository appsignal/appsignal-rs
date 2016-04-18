use std::env;
use std::path::Path;
use std::process::Command;

static AGENT_VERSION: &'static str = "4201306"; // Also present in lib.rs

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let out_path = Path::new(&out_dir);

    // Determine system architecture
    let target = env::var("TARGET").unwrap();
    let architecture = match target.as_str() {
        "x86_64-apple-darwin" => "x86_64-darwin",
        "x86_64-unknown-linux" => "x86_64-linux",
        "i686-unknown-linux" => "i686-linux",
        "x86-unknown-linux" => "i686-linux",
        arch => arch
    };

    // Download and extract agent
    let archive_name = format!(
        "appsignal-agent-{}-static.tar.gz",
        architecture
    );
    let url = format!(
        "https://appsignal-agent-releases.global.ssl.fastly.net/{}/{}",
        AGENT_VERSION,
        archive_name
    );
    assert!(Command::new("curl").arg("-O") // Save to disk
                                .arg("-L") // Follow redirects
                                .current_dir(&out_path)
                                .arg(url)
                                .status()
                                .unwrap()
                                .success());
    assert!(Command::new("tar").arg("xzf")
                               .arg(&archive_name)
                               .current_dir(&out_path)
                               .status()
                               .unwrap()
                               .success());

    // Let Cargo know it needs to link the library
    println!("cargo:rustc-link-search=native={}", &out_dir);
    println!("cargo:rustc-link-lib=static=appsignal");
}
