use std::{collections::HashSet, ops::Sub};

fn main() -> std::result::Result<(), std::io::Error> {
    let line = std::fs::read_to_string("input.txt")?;
    let notes = parse_input(&line);
    let answer1 = compute_part1(&notes);
    let answer2 = compute_part2(&notes);
    println!("part 1 answer is {}, part 2 answer is {}", answer1, answer2);
    Ok(())
}

struct NoteEntry {
    signal_patterns: Vec<HashSet<char>>,
    output_value: Vec<String>,
}

fn parse_note_entry(input: &str) -> NoteEntry {
    let (signal_patterns, output_value) = input.trim().split_once("|").unwrap();
    NoteEntry {
        signal_patterns: signal_patterns
            .split_ascii_whitespace()
            .map(|s| HashSet::from_iter(s.chars()))
            .collect(),
        output_value: output_value
            .split_ascii_whitespace()
            .map(|f| f.to_string())
            .collect(),
    }
}

#[test]
fn test_parse_note_entry() {
    // GIVEN
    let input =
        "acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf";

    // WHEN
    let note = parse_note_entry(input);

    // THEN
    assert_eq!(note.signal_patterns.len(), 10);
    assert_eq!(
        note.signal_patterns[0],
        HashSet::from_iter("acedgfb".chars())
    );
    assert_eq!(note.signal_patterns[9], HashSet::from_iter("ab".chars()));
    assert_eq!(note.output_value.len(), 4);
    assert_eq!(note.output_value[0], "cdfeb");
    assert_eq!(note.output_value[1], "fcadb");
    assert_eq!(note.output_value[2], "cdfeb");
    assert_eq!(note.output_value[3], "cdbaf");
}

fn parse_input(input: &str) -> Vec<NoteEntry> {
    input
        .trim()
        .split("\n")
        .into_iter()
        .map(|line| parse_note_entry(&line))
        .collect()
}

#[test]
fn test_parse_given_example_input() {
    // GIVEN
    let input =
        "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
        edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
        fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
        fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
        aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
        fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
        dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
        bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
        egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
        gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce";

    // WHEN
    let notes = parse_input(input);

    // THEN
    assert_eq!(notes.len(), 10);
}

/**
 * 0 -> 6 segments
 * 1 -> 2 segments
 * 2 -> 5 segments
 * 3 -> 5 segments
 * 4 -> 4 segments
 * 5 -> 5 segments
 * 6 -> 6 segments
 * 7 -> 3 segments
 * 8 -> 7 segments
 * 9 -> 6 segments
 *
 * 2 segments: [1]
 * 3 segments: [7]
 * 4 segments: [4]
 * 5 segments: [2, 3, 5]
 * 6 segments: [0, 6, 9]
 * 7 segments: [8]
 */
fn compute_part1(notes: &[NoteEntry]) -> u32 {
    notes
        .iter()
        .map(|note| {
            note.output_value
                .iter()
                .filter(|val| val.len() != 5 && val.len() != 6)
                .count() as u32
        })
        .sum()
}

#[allow(dead_code)] // all members come in handy when running tests
#[derive(Debug)]
struct WireSegmentMapping {
    top: char,
    bottom: char,
    top_left: char,
    top_right: char,
    bottom_left: char,
    bottom_right: char,
    middle: char,
}

fn get_digit(segment_count: usize, note: &NoteEntry) -> &HashSet<char> {
    note.signal_patterns
        .iter()
        .find(|sig| sig.len() == segment_count)
        .unwrap()
}

fn get_closest_digit<'a>(segments: &HashSet<char>, note: &'a NoteEntry) -> &'a HashSet<char> {
    note.signal_patterns
        .iter()
        .find(|sig| sig.sub(&segments).len() == 1)
        .unwrap()
}

//   I. (7 - 1) => top
//  II. (4 + 7) => closest is 9 => bottom
// III. (x: top+bottom+1) => closes is 3 => middle
//  IV. (4 - 3) => top_left
//   V. (8 - (4 + 3)) => bottom_left
//  VI. (y: top+middle+bottom+top_left+bottom_left) => closest is 6 => bottom_right
// VII. (1 - bottom_right) => top_right
fn infer_wire_segment_mapping(note: &NoteEntry) -> WireSegmentMapping {
    // I. (7 - 1) => top
    let one = get_digit(2, &note);
    let seven = get_digit(3, &note);
    let top = *seven.sub(&one).iter().next().unwrap();

    // II. (4 + 7) => closest is 9 => bottom
    let four = get_digit(4, &note);
    let four_and_seven = four.union(&seven).cloned().collect();
    let nine = get_closest_digit(&four_and_seven, &note);
    let bottom = *nine.sub(&four_and_seven).iter().next().unwrap();

    // III. (X: top+bottom+1) => closes is 3 => middle
    let x = HashSet::from_iter(vec![top, bottom]);
    let x: HashSet<char> = x.union(one).cloned().collect();
    let three = get_closest_digit(&x, &note);
    let middle = *three.sub(&x).iter().next().unwrap();

    // IV. (4 - 3) => top_left
    let top_left = *four.sub(&three).iter().next().unwrap();

    // V. (8 - (4 + 3)) => bottom_left
    let eight = get_digit(7, &note);
    let bottom_left = *eight.sub(&four).sub(&three).iter().next().unwrap();

    // VI. (y: top+middle+bottom+top_left+bottom_left) => closest is 6 => bottom_right
    let y = HashSet::from_iter(vec![top, middle, bottom, top_left, bottom_left]);
    let six = get_closest_digit(&y, &note);
    let bottom_right = *six.sub(&y).iter().next().unwrap();

    // VII. (1 - bottom_right) => top_right
    let top_right = *one
        .sub(&HashSet::from_iter(vec![bottom_right]))
        .iter()
        .next()
        .unwrap();

    WireSegmentMapping {
        top: top,
        bottom: bottom,
        middle: middle,
        top_left: top_left,
        bottom_left: bottom_left,
        bottom_right: bottom_right,
        top_right: top_right,
    }
}

#[test]
fn test_segment_inference() {
    // GIVEN
    let input =
        "acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf";

    // WHEN
    let note = parse_note_entry(input);
    let segment_mapping = infer_wire_segment_mapping(&note);

    assert_eq!(segment_mapping.top, 'd');
    assert_eq!(segment_mapping.bottom, 'c');
    assert_eq!(segment_mapping.middle, 'f');
    assert_eq!(segment_mapping.top_left, 'e');
    assert_eq!(segment_mapping.bottom_left, 'g');
    assert_eq!(segment_mapping.bottom_right, 'b');
    assert_eq!(segment_mapping.top_right, 'a');
}

fn decode_digit(digit: &str, wire_mapping: &WireSegmentMapping) -> u8 {
    if digit.len() == 2 {
        return 1;
    }
    if digit.len() == 3 {
        return 7;
    }
    if digit.len() == 4 {
        return 4;
    }
    if digit.len() == 7 {
        return 8;
    }
    if digit.len() == 5 && digit.contains(wire_mapping.top_left) {
        return 5;
    }
    if digit.len() == 5 && digit.contains(wire_mapping.bottom_left) {
        return 2;
    }
    if digit.len() == 5 {
        return 3;
    }
    if digit.len() == 6 && !digit.contains(wire_mapping.middle) {
        return 0;
    }
    if digit.len() == 6 && digit.contains(wire_mapping.bottom_left) {
        return 6;
    }
    return 9;
}

#[test]
fn test_example_digits_decoding() {
    // GIVEN
    let input =
        "acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf";

    // WHEN
    let note = parse_note_entry(input);
    let segment_mapping = infer_wire_segment_mapping(&note);
    let first_digit = decode_digit(&note.output_value[0], &segment_mapping);
    let second_digit = decode_digit(&note.output_value[1], &segment_mapping);
    let third_digit = decode_digit(&note.output_value[2], &segment_mapping);
    let fourth_digit = decode_digit(&note.output_value[3], &segment_mapping);

    // THEN
    assert_eq!(first_digit, 5);
    assert_eq!(second_digit, 3);
    assert_eq!(third_digit, 5);
    assert_eq!(fourth_digit, 3);
}

fn read_digits(note: &NoteEntry) -> u32 {
    let segment_mapping = infer_wire_segment_mapping(&note);
    let first_digit = decode_digit(&note.output_value[0], &segment_mapping) as u32;
    let second_digit = decode_digit(&note.output_value[1], &segment_mapping) as u32;
    let third_digit = decode_digit(&note.output_value[2], &segment_mapping) as u32;
    let fourth_digit = decode_digit(&note.output_value[3], &segment_mapping) as u32;
    return fourth_digit + third_digit * 10 + second_digit * 100 + first_digit * 1000;
}

#[test]
fn test_read_digits_on_first_example() {
    let input =
        "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
        edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
        fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
        fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
        aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
        fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
        dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
        bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
        egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
        gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce";
    let values = [8394, 9781, 1197, 9361, 4873, 8418, 4548, 1625, 8717, 4315];

    // WHEN
    let notes = parse_input(input);

    // THEN
    for (note, expected_value) in notes.iter().zip(values) {
        assert_eq!(read_digits(&note), expected_value);
    }
}

fn compute_part2(notes: &[NoteEntry]) -> u32 {
    notes.iter().map(|note| read_digits(&note)).sum()
}

#[test]
fn part_1_given_example() {
    let input =
        "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
        edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
        fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
        fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
        aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
        fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
        dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
        bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
        egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
        gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce";

    // WHEN
    let notes = parse_input(input);
    let r = compute_part1(&notes);

    // THEN
    assert!(r == 26);
}

#[test]
fn part_2_given_example() {
    let input =
        "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
        edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
        fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
        fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
        aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
        fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
        dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
        bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
        egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
        gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce";

    // WHEN
    let notes = parse_input(input);
    let r = compute_part2(&notes);

    // THEN
    assert!(r == 61229);
}
