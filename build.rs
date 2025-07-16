
fn main() {
    let out_dir = std::env::var("OUT_DIR").unwrap();
    let dest_assets = std::path::PathBuf::from(out_dir).join("assets");
    let src_assets = std::path::PathBuf::from("assets");

    if dest_assets.exists() {
        std::fs::remove_dir_all(&dest_assets).expect("Failed to remove existing assets directory");
    }

    std::fs::create_dir_all(&dest_assets).expect("Failed to create assets directory");
    for entry in std::fs::read_dir(src_assets).expect("Failed to read assets directory") {
        let entry = entry.expect("Failed to read entry in assets directory");
        let src_path = entry.path();
        if src_path.is_file() {
            let dest_path = dest_assets.join(src_path.file_name().expect("Failed to get file name"));
            std::fs::copy(&src_path, &dest_path).expect("Failed to copy asset file");
        }
    }
    println!("cargo:rerun-if-changed=assets");
}