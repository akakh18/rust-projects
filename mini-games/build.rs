extern crate winres;

use std::fs;
use std::path::Path;

fn main() {
    // --- Windows icon setup ---
    if cfg!(target_os = "windows") {
        let mut res = winres::WindowsResource::new();
        res.set_icon("icon.ico");
        res.compile().unwrap();
    }

    // --- Asset copying ---
    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    let asset_dir = Path::new(&manifest_dir).join("assets");

    if asset_dir.exists() {
        let out_dir = std::env::var("OUT_DIR").unwrap();
        let target_dir = Path::new(&out_dir).ancestors().nth(3).unwrap();

        for profile in ["debug", "release"] {
            let dest = target_dir.join(profile).join("assets");
            let _ = fs::remove_dir_all(&dest);
            fs::create_dir_all(&dest).unwrap();

            for entry in fs::read_dir(&asset_dir).unwrap() {
                let entry = entry.unwrap();
                fs::copy(entry.path(), dest.join(entry.file_name())).unwrap();
            }
        }
    }
}
