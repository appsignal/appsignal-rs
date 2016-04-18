use std::path::Path;
use std::process::Command;

pub static AGENT_VERSION: &'static str = "4201306";

pub fn download_agent_archive(target: &str, out_path: &Path) {
    // Determine system architecture
    let architecture = match target {
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
                                .current_dir(out_path)
                                .arg(url)
                                .status()
                                .unwrap()
                                .success());
    assert!(Command::new("tar").arg("xzf")
                               .arg(&archive_name)
                               .current_dir(out_path)
                               .status()
                               .unwrap()
                               .success());
}
