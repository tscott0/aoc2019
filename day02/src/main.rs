mod part_1;
mod part_2;

extern crate utilities;

fn main() {
    println!(
        "DAY 2 part 1: {}",
        part_1::solve(part_1::restore_gravity(utilities::string_from_file(
            "input"
        )))
    );

    println!(
        "DAY 2 part 2: {}",
        part_2::solve(utilities::string_from_file("input"))
    );
}
