fn main() -> std::result::Result<(), std::io::Error> {
    let content = std::fs::read_to_string("input.txt")?;
    let measurements: Vec<u32> = content
        .split("\n")
        .map(|l| l.parse::<u32>().unwrap())
        .collect();
    let answer1 = compute_part1(&measurements);
    let answer2 = compute_part2(&measurements);
    println!("part 1 answer is {}, part 2 answer is {}", answer1, answer2);
    Ok(())
}

fn compute_part1(measurements: &Vec<u32>) -> u32 {
    let downsteps: u32 = measurements[1..]
        .iter()
        .zip(measurements[..measurements.len() - 1].iter())
        .map(|(d2, d1)| if d2 > d1 { 1 } else { 0 })
        .sum();
    return downsteps;
}

fn compute_part2(measurements: &Vec<u32>) -> u32 {
    let three_measurement_sums: Vec<u32> = measurements[2..]
        .iter()
        .zip(measurements[1..].iter())
        .zip(measurements.iter())
        .map(|((ip2, ip1), ip0)| ip2 + ip1 + ip0)
        .collect();

    let downsteps: u32 = three_measurement_sums[1..]
        .iter()
        .zip(three_measurement_sums[..three_measurement_sums.len() - 1].iter())
        .map(|(d2, d1)| if d2 > d1 { 1 } else { 0 })
        .sum();
    downsteps
}

#[test]
fn part_1_given_example() {
    // GIVEN
    let input = [199, 200, 208, 210, 200, 207, 240, 269, 260, 263];

    // WHEN
    let r = compute_part1(&input.to_vec());

    // THEN
    assert!(r == 7)
}

#[test]
fn part_2_given_example() {
    // GIVEN
    let input = [199, 200, 208, 210, 200, 207, 240, 269, 260, 263];

    // WHEN
    let r = compute_part2(&input.to_vec());

    // THEN
    assert!(r == 5)
}
