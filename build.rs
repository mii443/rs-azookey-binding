fn main() {
    let project_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    println!(
        "cargo:rustc-link-search={}/azookey-swift/.build/release/",
        project_dir
    );
    println!("cargo:rustc-link-lib=azookey-swift");
}
