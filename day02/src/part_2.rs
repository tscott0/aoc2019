pub fn solve(input: String) -> String {
    let codes = input
        .split(",")
        .map(|x| x.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();

    for noun in 0..=99 {
        for verb in 0..=99 {
            let mut attempt = codes.clone();
            attempt[1] = noun;
            attempt[2] = verb;

            let mut finished = false;
            attempt.clone().chunks(4).for_each(|c| {
                if !finished {
                    if c[0] == 99 {
                        finished = true;
                        return;
                    }

                    match c[0] {
                        1 => attempt[c[3]] = attempt[c[1]] + attempt[c[2]],
                        2 => attempt[c[3]] = attempt[c[1]] * attempt[c[2]],
                        _ => panic!("Unexpected OP code {}", c[0]), // ERROR
                    }
                }
            });

            if attempt[0] == 19690720 {
                return format!("{},{} = {}{}", noun, verb, noun, verb);
            }
        }
    }

    "UNKNOWN".to_string()
}
