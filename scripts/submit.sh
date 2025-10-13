#!/usr/local/bin/fish

# Check if argument is provided
if test (count $argv) -eq 0
    echo "Usage: $_ <problem_name>"
    echo "Example: $_ cows_macro"
    exit 1
end
# -g + -x makes it available as env-var within the script.
set -gx PROBLEM $argv[1]
set submit_file "submit.rs"

# Compile down to rust single-file for submission on grading servers that support Rust.
# 1. Expand the utilities
cargo expand --lib problems::util > $submit_file

# 2. Expand the problem solution, and:
#   (a) remove cp_macros usage
#   (b) remove `cases/` prefix as grading server has the input file in the root directory.
#   (c) reference util module in the file and not the library crate.
cargo expand --lib problems::$PROBLEM \
    | sed '/use cp_macros::/d' \
    | sed 's/cases\///g' \
    | sed 's/crate::problems::util/super::util/g' >> $submit_file

echo "
fn main() {
    $PROBLEM::run_problem();
}
" >> $submit_file
