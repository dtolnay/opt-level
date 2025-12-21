use std::env;
use std::fs;
use std::path::Path;

fn main() {
    let out_dir = env::var_os("OUT_DIR").unwrap();
    let opt_level = env::var("OPT_LEVEL").unwrap();
    let dest_path = Path::new(&out_dir).join("opt-level");
    fs::write(dest_path, opt_level).unwrap();
}
