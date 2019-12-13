mod part_1;
mod part_2;

extern crate utilities;

fn main() {
    println!(
        "DAY X part 1: {}",
        part_1::solve(utilities::string_from_file("input"))
    );

    println!(
        "DAY X part 2: {}",
        part_2::solve(utilities::string_from_file("input"))
    );
}
