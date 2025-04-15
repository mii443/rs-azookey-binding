fn main() {
    let project_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();

    // Build the Swift library
    let swift_build_command = format!(
        "swift build -c release --package-path {}/azookey-swift",
        project_dir
    );
    if cfg!(target_os = "windows") {
        let output = std::process::Command::new("cmd")
            .args(&["/C", &swift_build_command])
            .output()
            .expect("Failed to execute command");
        if !output.status.success() {
            panic!(
                "Swift build failed: {}",
                String::from_utf8_lossy(&output.stderr)
            );
        }

        // rename the output file to azookey-swift.lib from libazookey-swift.a
        let output_path = format!(
            "{}/azookey-swift/.build/release/libazookey-swift.a",
            project_dir
        );
        let new_output_path = format!(
            "{}/azookey-swift/.build/release/azookey-swift.lib",
            project_dir
        );
        std::fs::rename(&output_path, &new_output_path).expect("Failed to rename file");
    } else {
        // non-windows
        let output = std::process::Command::new("sh")
            .arg("-c")
            .arg(&swift_build_command)
            .output()
            .expect("Failed to execute command");
        if !output.status.success() {
            panic!(
                "Swift build failed: {}",
                String::from_utf8_lossy(&output.stderr)
            );
        }
    }

    // Set the Swift library path
    println!(
        "cargo:rustc-link-search={}/azookey-swift/.build/release/",
        project_dir
    );
    println!(
        "cargo:rustc-link-search={}",
        std::env::var("SWIFT_LIB_DIR").unwrap()
    );
    println!("cargo:rustc-link-lib=static=azookey-swift");
}
