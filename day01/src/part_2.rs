pub fn solve(input: String) -> i64 {
    let lines = input.split("\n");

    let mut total_fuel: i64 = 0;

    for l in lines {
        let mass = l.parse::<i64>().unwrap();

        let initial_fuel = fuel(mass) as i64;
        total_fuel += initial_fuel;

        let mut fuel_calc = initial_fuel;
        loop {
            fuel_calc = fuel(fuel_calc);

            if fuel_calc == 0 {
                break;
            }
            total_fuel += fuel_calc as i64;
        }
    }
    total_fuel
}

pub fn fuel(mass: i64) -> i64 {
    let calc: i64 = (mass / 3) - 2;
    println!("Mass: {}", mass);
    if calc < 0 {
        return 0;
    }
    return calc;
}

#[test]
fn example_1() {
    assert_eq!(solve(String::from("14")), 2);
    assert_eq!(solve(String::from("1969")), 966);
    assert_eq!(solve(String::from("100756")), 50346);
}
