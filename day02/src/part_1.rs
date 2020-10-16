// Refactored to solve in a single function
pub fn solve(input: String) -> String {
    let mut finished = false;
    let mut codes = input
        .split(",")
        .map(|x| x.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();

    codes.clone().chunks(4).for_each(|c| {
        if !finished {
            if c[0] == 99 {
                finished = true;
                return;
            }

            match c[0] {
                1 => codes[c[3]] = codes[c[1]] + codes[c[2]],
                2 => codes[c[3]] = codes[c[1]] * codes[c[2]],
                _ => panic!("Unexpected OP code {}", c[0]), // ERROR
            }
        }
    });

    codes
        .iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(",")
}

pub fn restore_gravity(s: String) -> String {
    let mut codes = s
        .split(",")
        .map(|x| x.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();

    codes[1] = 12;
    codes[2] = 2;

    codes
        .iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(",")
}

// The original implementation
pub fn solve1(input: String) -> String {
    let mut codes: Vec<usize> = input
        .split(",")
        .map(|x| x.parse::<usize>().unwrap())
        .collect();

    let mut cursor: usize = 0;

    loop {
        if finished(&mut cursor, &mut codes) {
            break;
        }
    }

    codes_to_string(&mut codes)
}

fn finished(cursor: &mut usize, codes: &mut Vec<usize>) -> bool {
    let op_code = codes[*cursor];
    if op_code == 99 {
        return true;
    }

    let a = codes[codes[*cursor + 1]];
    let b = codes[codes[*cursor + 2]];
    let result_pos = codes[*cursor + 3];

    let op = if op_code == 1 { "+" } else { "*" };
    println!(
        "cursor={} op_code={} {}{}{} into [{}]",
        cursor, op_code, a, op, b, result_pos
    );
    match op_code {
        1 => codes[result_pos] = a + b,
        2 => codes[result_pos] = a * b,
        _ => panic!("Unexpected OP code"), // ERROR
    }
    println!("{}", codes_to_string(codes));

    *cursor += 4;
    false
}

fn codes_to_string(codes: &mut Vec<usize>) -> String {
    codes
        .iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(",")
}

#[test]
fn example_1() {
    assert_eq!(solve(String::from("1,0,0,0,99")), "2,0,0,0,99");
}

#[test]
fn example_2() {
    assert_eq!(solve(String::from("2,3,0,3,99")), "2,3,0,6,99");
}

#[test]
fn example_3() {
    assert_eq!(solve(String::from("2,4,4,5,99,0")), "2,4,4,5,99,9801");
}
