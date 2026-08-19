use std::fs;
use std::path::Path;

fn watch_dir(path: &Path) {
    println!("cargo:rerun-if-changed={}", path.display());

    for entry in fs::read_dir(path).expect("read watched directory") {
        let path = entry.expect("read migration directory entry").path();
        if path.is_dir() {
            watch_dir(&path);
        } else {
            println!("cargo:rerun-if-changed={}", path.display());
        }
    }
}

fn main() {
    watch_dir(Path::new("migrations"));
}
