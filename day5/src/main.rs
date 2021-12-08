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
                // segments should always point away radially from centre
                start: (x1.min(x2), y1.min(y2)),
                end: (x2.max(x1), y2.max(y1)),
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
    assert!(vents[9].start == (5, 2));
    assert!(vents[9].end == (8, 5));
}

fn is_horizontal_segment(segment: &Segment) -> bool {
    segment.start.1 == segment.end.1
}

fn is_vertical_segment(segment: &Segment) -> bool {
    segment.start.0 == segment.end.0
}

fn horiz_vert_intersection(horiz_segment: &Segment, vert_segment: &Segment) -> Option<(u32, u32)> {
    let horiz_left = horiz_segment.start.0.min(horiz_segment.end.0);
    let horiz_right = horiz_segment.end.0.max(horiz_segment.start.0);
    let vert_top = vert_segment.start.1.min(vert_segment.end.1);
    let vert_bottom = vert_segment.end.1.max(vert_segment.start.1);
    if (horiz_left <= vert_segment.start.0 && vert_segment.start.0 <= horiz_right)
        && (vert_top <= horiz_segment.start.1 && horiz_segment.start.1 <= vert_bottom)
    {
        Some((vert_segment.start.0, horiz_segment.start.1))
    } else {
        None
    }
}

#[test]
fn test_horiz_vert_intersection() {
    // GIVEN
    let seg_a = Segment {
        start: (0, 4),
        end: (5, 4),
    };
    let seg_b = Segment {
        start: (3, 2),
        end: (3, 6),
    };

    // WHEN
    let intersect = horiz_vert_intersection(&seg_a, &seg_b);

    // THEN
    assert!(intersect.is_some());
    assert!(intersect.unwrap() == (3, 4));
}

fn same_direction_overlap(seg_a: &Segment, seg_b: &Segment) -> Option<Segment> {
    if is_horizontal_segment(seg_a) && seg_a.start.1 == seg_b.start.1 {
        let left_a = seg_a.start.0;
        let left_b = seg_b.start.0;
        let right_a = seg_a.end.0;
        let right_b = seg_b.end.0;
        if right_a.min(right_b) >= left_a.max(left_b) {
            return Some(Segment {
                start: (left_a.max(left_b), seg_a.start.1),
                end: (right_a.min(right_b), seg_a.start.1),
            });
        }
    }
    if is_vertical_segment(seg_a) && seg_a.start.0 == seg_b.start.0 {
        let top_a = seg_a.start.1;
        let top_b = seg_b.start.1;
        let bottom_a = seg_a.end.1;
        let bottom_b = seg_b.end.1;
        if bottom_a.min(bottom_b) >= top_a.max(top_b) {
            return Some(Segment {
                start: (seg_a.start.0, top_a.max(top_b)),
                end: (seg_a.start.0, bottom_a.min(bottom_b)),
            });
        }
    }
    return None;
}

fn overlap_points_from_segment(segment: &Segment) -> Vec<(u32, u32)> {
    if is_horizontal_segment(segment) {
        return (segment.start.0..segment.end.0 + 1)
            .map(|n| (n, segment.start.1))
            .collect();
    }
    if is_vertical_segment(segment) {
        return (segment.start.1..segment.end.1 + 1)
            .map(|n| (segment.start.0, n))
            .collect();
    }

    vec![]
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
    assert!(overlap.is_some());
    let overlap_points = overlap_points_from_segment(&overlap.unwrap());

    // THEN
    assert!(overlap_points.len() == 3);
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
    assert!(overlap.is_some());
    let overlap_points = overlap_points_from_segment(&overlap.unwrap());

    // THEN
    assert!(overlap_points.len() == 6);
}

#[test]
fn test_same_direction_overlap_segment_and_single_point() {
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
    assert!(overlap.is_some());
    let overlap_points = overlap_points_from_segment(&overlap.unwrap());

    // THEN
    assert!(overlap_points.len() == 1);
}

fn compute_part1(vents: &[Segment]) -> u32 {
    // keeping only horizontal and vertical lines
    let vents: Vec<&Segment> = vents
        .iter()
        .filter(|seg| is_horizontal_segment(seg) || is_vertical_segment(seg))
        .collect();

    let mut intersection_points = HashSet::new();
    for (index, segment) in vents.iter().enumerate() {
        if index == vents.len() - 1 {
            break;
        }
        for other_segment in vents[index + 1..].iter() {
            if is_horizontal_segment(segment) && is_vertical_segment(other_segment) {
                if let Some(point) = horiz_vert_intersection(segment, other_segment) {
                    intersection_points.insert(point);
                }
            } else if is_horizontal_segment(other_segment) && is_vertical_segment(segment) {
                if let Some(point) = horiz_vert_intersection(other_segment, segment) {
                    intersection_points.insert(point);
                }
            } else {
                let overlap = same_direction_overlap(segment, other_segment);
                if let Some(overlap) = overlap {
                    let overlap_points = overlap_points_from_segment(&overlap);
                    for n in overlap_points.iter() {
                        intersection_points.insert(*n);
                    }
                }
            }
        }
    }
    intersection_points.len() as u32
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
    assert!(r == 5)
}

#[test]
fn part_2_given_example() {
    // GIVEN
    // WHEN
    // THEN
    // assert!(r == 1924)
}
