// macro_rules! use_problem {
//     ($m:expr) => {
//         inner! { $m }
//     };
// }

// macro_rules! inner {
//     ($problem:literal) => {
//         use challenges::problems::$problem::run_problem;
//     }
// }
//
// macro_rules! use_problem {
//     ($problem:literal) => {
//         use challenges::problems::$problem::run_problem;
//     };
// }

// use_problem!("kayak");

// problem_spec.rs is defined in the build file as I couldn't get a way
// to get `use` statements in macros.
include!(concat!(env!("OUT_DIR"), "/problem_spec.rs"));

fn main() {
    run_problem();
}
