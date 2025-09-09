include!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/src/problems/",
    env!("PROBLEM"),
    ".rs"
));

fn main() {
    run_problem();
}
