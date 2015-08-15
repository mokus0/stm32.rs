use std::env;
use std::fs;
use std::path::Path;

fn main() {
    let in_path = Path::new("ld");
    
    let out_dir = env::var("OUT_DIR").unwrap();
    let out_path = Path::new(&out_dir);
    
    for entry in fs::read_dir(&in_path).unwrap() {
        let ld_script = entry.unwrap();
        fs::copy(ld_script.path(), out_path.join(ld_script.file_name()));
    }
    
    println!("cargo:rustc-link-search=native={}", out_dir);
}