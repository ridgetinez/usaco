use std::env;
use std::fs;
use std::path::Path;

fn main() {
    // Weirdly necessary when developing in an editor that runs `cargo` commands
    // during development / when files change. Otherwise specifying the problem
    // explicitly will trigger the editor to another cargo command and overwrite
    // the changes back to the default problem.
    println!("cargo::rerun-if-env-changed=PROBLEM");

    let out_dir = env::var_os("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("problem_spec.rs");
    let problem_name = env::var("PROBLEM").unwrap_or("kayak".to_string());
    fs::write(
        &dest_path,
        format!("use challenges::problems::{}::run_problem;\n", problem_name),
    )
    .unwrap();
}
