// build.rs
use rust_version_info_file::rust_version_info_file;

fn main() {
    let path = {
        #[cfg(feature = "debian_build")]
        let dir = "target".to_string();
        #[cfg(not(feature = "debian_build"))]
        let dir = std::env::var("OUT_DIR").expect("Environment variable OUT_DIR must be set. Check your build environment.");
        //
        format!("{dir}/rust-version-info.txt")
    };
    rust_version_info_file(path.as_str(), "Cargo.toml");
}
