use std::env;
use std::fs;
use std::path::Path;

fn main() {
    let out_dir = env::var_os("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("problem_spec.rs");
    fs::write(
        &dest_path,
        format!("use challenges::problems::{}::run_problem;\n", env::var_os("PROBLEM").unwrap().to_str().unwrap())
    ).unwrap();
    println!("cargo::rerun-if-changed=build.rs");
}
