fn main() -> std::result::Result<(), std::io::Error> {
    let line = std::fs::read_to_string("input.txt")?;
    let ages = parse_input(&line);
    let answer1 = compute_part1(&ages);
    let answer2 = compute_part2(&ages);
    println!("part 1 answer is {}, part 2 answer is {}", answer1, answer2);
    Ok(())
}

fn parse_input(input: &str) -> Vec<u32> {
    input
        .split(",")
        .map(|f| f.parse::<u32>().unwrap())
        .collect()
}

#[test]
fn test_parse_given_example_input() {
    // GIVEN
    let input = "16,1,2,0,4,2,7,1,2,14";

    // WHEN
    let positions = parse_input(input);

    // THEN
    assert!(positions == vec![16, 1, 2, 0, 4, 2, 7, 1, 2, 14]);
}

fn compute_part1(positions: &[u32]) -> u32 {
    // find min of f(x) = sum(abs(Ki-x))
    let mut positions = positions.to_vec();
    positions.sort();
    let x_min = positions[positions.len() / 2];
    positions
        .iter()
        .map(|k| (*k as i32 - x_min as i32).abs() as u32)
        .sum()
}

fn compute_part2(positions: &[u32]) -> u32 {
    0
}

#[test]
fn part_1_given_example() {
    let input = "16,1,2,0,4,2,7,1,2,14";

    // WHEN
    let positions = parse_input(input);
    let r = compute_part1(&positions);

    // THEN
    assert!(r == 37);
}

#[test]
fn part_2_given_example() {}
