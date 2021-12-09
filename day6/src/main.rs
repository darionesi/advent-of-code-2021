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
    let input = "3,4,3,1,2";

    // WHEN
    let ages = parse_input(input);

    // THEN
    assert!(ages.len() == 5);
    assert!(ages == vec![3, 4, 3, 1, 2]);
}

fn grow_lanternfish(ages: &[u32], days: u32) -> u64 {
    let mut ages_count = [0u64; 9];
    for &fish in ages.iter() {
        ages_count[fish as usize] += 1;
    }
    for _ in 0..days {
        let about_to_create_new_fish = ages_count[0];
        ages_count[0] = ages_count[1];
        ages_count[1] = ages_count[2];
        ages_count[2] = ages_count[3];
        ages_count[3] = ages_count[4];
        ages_count[4] = ages_count[5];
        ages_count[5] = ages_count[6];
        ages_count[6] = ages_count[7] + about_to_create_new_fish;
        ages_count[7] = ages_count[8];
        ages_count[8] = about_to_create_new_fish;
    }
    ages_count.iter().sum()
}

fn compute_part1(ages: &[u32]) -> u64 {
    grow_lanternfish(ages, 80)
}

fn compute_part2(ages: &[u32]) -> u64 {
    grow_lanternfish(ages, 256)
}

#[test]
fn test_grow_lanternfish() {
    // GIVEN
    let ages = vec![3u32, 4, 3, 1, 2];

    // WHEN
    let r = grow_lanternfish(&ages, 18);

    // THEN
    assert!(r == 26);
}

#[test]
fn part_1_given_example() {
    // GIVEN
    let input = "3,4,3,1,2";

    // WHEN
    let ages = parse_input(input);
    let r = compute_part1(&ages);

    // THEN
    assert!(r == 5934);
}

#[test]
fn part_2_given_example() {
    // GIVEN
    let input = "3,4,3,1,2";

    // WHEN
    let ages = parse_input(input);
    let r = compute_part2(&ages);

    // THEN
    assert!(r == 26984457539);
}
