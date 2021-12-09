fn main() -> std::result::Result<(), std::io::Error> {
    let input = std::fs::read_to_string("input.txt")?;

    let input = parse_input(&input);

    let answer1 = compute_part1(&input.0, input.1);
    let answer2 = compute_part2(&input.0, input.1);
    println!("part 1 answer is {}, part 2 answer is {}", answer1, answer2);
    Ok(())
}

fn parse_input(input: &str) -> (Vec<u32>, usize) {
    let lines: Vec<&str> = input.split("\n").collect();
    let bits = lines[0].len();
    let vec = lines
        .iter()
        .map(|&line| u32::from_str_radix(line, 2).unwrap())
        .collect();
    (vec, bits)
}

fn compute_part1(input: &Vec<u32>, bits_available: usize) -> u32 {
    let mut gamma_rate = 0;
    let mut epsilon_rate = 0;
    for bit_index in 0..bits_available {
        let occurrences_of_1: u32 = input.iter().map(|n| (n >> bit_index) & 1).sum();
        if occurrences_of_1 > (input.len() as u32 / 2) {
            gamma_rate += 1 << bit_index;
        } else {
            epsilon_rate += 1 << bit_index;
        }
    }
    gamma_rate * epsilon_rate
}

fn compute_part2(input: &Vec<u32>, bits_available: usize) -> u32 {
    let mut possible_oxygen_gen_rating = input.clone();
    for bit_index in 0..bits_available {
        let bit_index = bits_available - bit_index - 1;
        let occurrences_of_1: u32 = possible_oxygen_gen_rating
            .iter()
            .map(|n| (n >> bit_index) & 1)
            .sum();
        let mask = 1 << bit_index;
        let expected =
            if occurrences_of_1 >= (possible_oxygen_gen_rating.len() as u32 - occurrences_of_1) {
                mask
            } else {
                0
            };
        possible_oxygen_gen_rating = possible_oxygen_gen_rating
            .into_iter()
            .filter(|n| (n & mask) == expected)
            .collect();
        if possible_oxygen_gen_rating.len() == 1 {
            break;
        }
    }
    let mut possible_co2_scrubber_rating = input.clone();
    for bit_index in 0..bits_available {
        let bit_index = bits_available - bit_index - 1;
        let occurrences_of_1: u32 = possible_co2_scrubber_rating
            .iter()
            .map(|n| (n >> bit_index) & 1)
            .sum();
        let mask = 1 << bit_index;
        let expected =
            if occurrences_of_1 >= (possible_co2_scrubber_rating.len() as u32 - occurrences_of_1) {
                0
            } else {
                mask
            };
        possible_co2_scrubber_rating = possible_co2_scrubber_rating
            .into_iter()
            .filter(|n| (n & mask) == expected)
            .collect();
        if possible_co2_scrubber_rating.len() == 1 {
            break;
        }
    }
    possible_oxygen_gen_rating[0] * possible_co2_scrubber_rating[0]
}

#[test]
fn part1_given_example() {
    let input = [
        0b00100, 0b11110, 0b10110, 0b10111, 0b10101, 0b01111, 0b00111, 0b11100, 0b10000, 0b11001,
        0b00010, 0b01010,
    ];
    let n_bits = 5;

    // WHEN
    let answer = compute_part1(&input.to_vec(), n_bits);

    // THEN
    assert!(answer == 198);
}

#[test]
fn part2_given_example() {
    let input = [
        0b00100, 0b11110, 0b10110, 0b10111, 0b10101, 0b01111, 0b00111, 0b11100, 0b10000, 0b11001,
        0b00010, 0b01010,
    ];
    let n_bits = 5;

    // WHEN
    let answer = compute_part2(&input.to_vec(), n_bits);

    // THEN
    assert!(answer == 230);
}
