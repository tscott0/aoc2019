pub fn solve(input: String) -> u32 {
    let lines = input.split("\n");

    let mut total_fuel: u32 = 0;

    for l in lines {
        let mass = l.parse::<u32>().unwrap();

        println!("Mass: {}", mass);
        total_fuel += fuel(mass)
    }
    total_fuel
}

pub fn fuel(mass: u32) -> u32 {
    (mass / 3) - 2
}

#[test]
fn example_1() {
    assert_eq!(solve(String::from("12")), 2);
    assert_eq!(solve(String::from("14")), 2);
    assert_eq!(solve(String::from("1969")), 654);
    assert_eq!(solve(String::from("100756")), 33583);
}
