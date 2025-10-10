// problem_spec.rs is defined in the build file as I couldn't get a way
// to get `use` statements in macros.
include!(concat!(env!("OUT_DIR"), "/problem_spec.rs"));

fn main() {
    run_problem();
}
