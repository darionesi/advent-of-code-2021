use std::{
    collections::{HashMap, HashSet},
    fmt::Debug,
    iter::{self, Enumerate},
};

fn main() -> std::result::Result<(), std::io::Error> {
    let lines = std::fs::read_to_string("input.txt")?;
    let bingo_game = parse_input(lines.split("\n").collect());
    let answer1 = compute_part1(&bingo_game);
    let answer2 = compute_part2(&bingo_game);
    println!("part 1 answer is {}, part 2 answer is {}", answer1, answer2);
    Ok(())
}

#[derive(Debug)]
struct Segment {
    start: (u32, u32),
    end: (u32, u32),
}

fn parse_input(lines: Vec<&str>) -> Vec<Segment> {
    lines
        .into_iter()
        .map(|segment_line| {
            let (x1, y1, x2, y2): (u32, u32, u32, u32);
            text_io::scan!(segment_line.trim().bytes() => "{},{} -> {},{}", x1, y1, x2, y2);
            Segment {
                start: (x1, y1),
                end: (x2, y2),
            }
        })
        .collect()
}

#[test]
fn test_parse_given_example_input() {
    // GIVEN
    let input = "0,9 -> 5,9
    8,0 -> 0,8
    9,4 -> 3,4
    2,2 -> 2,1
    7,0 -> 7,4
    6,4 -> 2,0
    0,9 -> 2,9
    3,4 -> 1,4
    0,0 -> 8,8
    5,5 -> 8,2";
    let lines: Vec<&str> = input.split("\n").collect();

    // WHEN
    let vents = parse_input(lines);

    // THEN
    assert!(vents.len() == 10);
    assert!(vents[0].start == (0, 9));
    assert!(vents[0].end == (5, 9));
    assert!(vents[9].start == (5, 5));
    assert!(vents[9].end == (8, 2));
}

fn is_horizontal_segment(segment: &Segment) -> bool {
    segment.start.1 == segment.end.1
}

fn is_vertical_segment(segment: &Segment) -> bool {
    segment.start.0 == segment.end.0
}

fn is_horiz_vert_intersection(horiz_segment: &Segment, vert_segment: &Segment) -> bool {
    let horiz_left = horiz_segment.start.0;
    let horiz_right = horiz_segment.end.0;
    let vert_top = vert_segment.start.1;
    let vert_bottom = vert_segment.end.1;
    if (horiz_left <= vert_segment.start.0 && vert_segment.start.0 <= horiz_right)
        && (vert_top <= horiz_segment.start.1 && horiz_segment.start.1 <= vert_bottom)
    {
        true
    } else {
        false
    }
}

fn same_direction_overlap(seg_a: &Segment, seg_b: &Segment) -> u32 {
    if is_horizontal_segment(seg_a) {
        let left_a = seg_a.start.0;
        let left_b = seg_b.start.0;
        let right_a = seg_a.end.0;
        let right_b = seg_b.end.0;
        if right_a.min(right_b) > left_a.max(left_b) {
            return right_a.min(right_b) - left_a.max(left_b) + 1;
        }
    }
    if is_vertical_segment(seg_a) {
        let top_a = seg_a.start.1;
        let top_b = seg_b.start.1;
        let bottom_a = seg_a.end.1;
        let bottom_b = seg_b.end.1;
        if bottom_a.min(bottom_b) > top_a.max(top_b) {
            return bottom_a.min(bottom_b) - top_a.max(top_b) + 1;
        }
    }
    0
}

#[test]
fn test_same_direction_overlap() {
    // GIVEN
    let seg_a = Segment {
        start: (0, 9),
        end: (5, 9),
    };
    let seg_b = Segment {
        start: (0, 9),
        end: (2, 9),
    };

    // WHEN
    let overlap = same_direction_overlap(&seg_a, &seg_b);

    // THEN
    assert!(overlap == 3);
}

#[test]
fn test_same_direction_overlap_same_segment() {
    // GIVEN
    let seg_a = Segment {
        start: (0, 9),
        end: (5, 9),
    };

    // WHEN
    let overlap = same_direction_overlap(&seg_a, &seg_a);

    // THEN
    assert!(overlap == 6);
}

#[test]
fn test_same_direction_overlap_segment_and_single() {
    // GIVEN
    let seg_a = Segment {
        start: (0, 9),
        end: (5, 9),
    };
    let seg_b = Segment {
        start: (0, 9),
        end: (0, 9),
    };

    // WHEN
    let overlap = same_direction_overlap(&seg_a, &seg_b);

    // THEN
    assert!(overlap == 1);
}

fn compute_part1(vents: &[Segment]) -> u32 {
    // keeping only horizontal and vertical lines
    let vents: Vec<&Segment> = vents
        .iter()
        .filter(|seg| is_horizontal_segment(seg) || is_vertical_segment(seg))
        .collect();

    let mut acc = 0u32;
    for (index, segment) in vents.iter().enumerate() {
        if index == vents.len() - 1 {
            break;
        }
        for other_segment in vents[index + 1..].iter() {
            if is_horizontal_segment(segment)
                && is_vertical_segment(other_segment)
                && is_horiz_vert_intersection(segment, other_segment)
            {
                acc += 1;
                println!("Segment {:?} and {:?} intersect", segment, other_segment);
            } else if is_horizontal_segment(other_segment)
                && is_vertical_segment(segment)
                && is_horiz_vert_intersection(other_segment, segment)
            {
                acc += 1;
                println!("Segment {:?} and {:?} intersect", segment, other_segment);
            } else {
                let overlap = same_direction_overlap(segment, other_segment);
                println!(
                    "Segment {:?} and {:?} overlap by {}",
                    segment, other_segment, overlap
                );
                acc += overlap;
            }
        }
    }
    acc
}

fn compute_part2(vents: &[Segment]) -> u32 {
    0
}

#[test]
fn part_1_given_example() {
    // GIVEN
    let input = "0,9 -> 5,9
    8,0 -> 0,8
    9,4 -> 3,4
    2,2 -> 2,1
    7,0 -> 7,4
    6,4 -> 2,0
    0,9 -> 2,9
    3,4 -> 1,4
    0,0 -> 8,8
    5,5 -> 8,2";

    // WHEN
    let vents = parse_input(input.split("\n").collect());
    let r = compute_part1(&vents);

    // THEN
    println!("r={}", r);
    assert!(r == 5)
}

#[test]
fn part_2_given_example() {
    // GIVEN
    let input = "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

    22 13 17 11  0
     8  2 23  4 24
    21  9 14 16  7
     6 10  3 18  5
     1 12 20 15 19
    
     3 15  0  2 22
     9 18 13 17  5
    19  8  7 25 23
    20 11 10 24  4
    14 21 16 12  6
    
    14 21 17 24  4
    10 16 15  9 19
    18  8 23 26 20
    22 11 13  6  5
     2  0 12  3  7";

    // WHEN
    let vents = parse_input(input.split("\n").collect());
    let r = compute_part2(&vents);

    // THEN
    // assert!(r == 1924)
}
