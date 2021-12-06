fn main() -> std::result::Result<(), std::io::Error> {
    let input = std::fs::read_to_string("input.txt")?;

    let course = parse_course_input(&input);

    let answer1 = compute_part1(&course);
    let answer2 = compute_part2(&course);
    println!("part 1 answer is {}, part 2 answer is {}", answer1, answer2);
    Ok(())
}

fn parse_course_input(input: &str) -> Vec<(i32, i32)> {
    input
        .split("\n")
        .map(|line| {
            let tokens: Vec<&str> = line.split(" ").collect();
            let (direction, amount) = (tokens[0], tokens[1].parse::<i32>().unwrap());
            match direction {
                "forward" => (amount, 0),
                "up" => (0, -amount),
                "down" => (0, amount),
                _ => (0, 0),
            }
        })
        .collect()
}

fn compute_part1(course: &Vec<(i32, i32)>) -> i32 {
    let final_pos = course
        .iter()
        .fold((0 /* horiz pos */, 0 /* depth */), |acc, step| {
            (acc.0 + step.0, acc.1 + step.1)
        });
    final_pos.0 * final_pos.1
}

fn compute_part2(course: &Vec<(i32, i32)>) -> i64 {
    let final_pos = course.iter().fold(
        (
            0i64, /* horiz pos */
            0i64, /* depth */
            0i64, /* aim */
        ),
        |acc, step| {
            (
                acc.0 + step.0 as i64,
                acc.1 + step.0 as i64 * acc.2,
                acc.2 + step.1 as i64,
            )
        },
    );
    final_pos.0 * final_pos.1
}

#[test]
fn part1_parse_input() {
    // GIVEN
    let input = [
        "forward 5",
        "down 5",
        "forward 8",
        "up 3",
        "down 8",
        "forward 2",
    ];

    // WHEN
    let course = parse_course_input(&input.join("\n"));

    // THEN
    assert!(course.len() == 6);
    assert!(course[0] == (5, 0));
    assert!(course[1] == (0, 5));
    assert!(course[2] == (8, 0));
    assert!(course[3] == (0, -3));
    assert!(course[4] == (0, 8));
    assert!(course[5] == (2, 0));
}

#[test]
fn part1_given_example() {
    let input = [
        "forward 5",
        "down 5",
        "forward 8",
        "up 3",
        "down 8",
        "forward 2",
    ];

    // WHEN
    let course = parse_course_input(&input.join("\n"));
    let answer = compute_part1(&course);

    // THEN
    assert!(answer == 150);
}

#[test]
fn part2_given_example() {
    let input = [
        "forward 5",
        "down 5",
        "forward 8",
        "up 3",
        "down 8",
        "forward 2",
    ];

    // WHEN
    let course = parse_course_input(&input.join("\n"));
    let answer = compute_part2(&course);

    // THEN
    assert!(answer == 900);
}
