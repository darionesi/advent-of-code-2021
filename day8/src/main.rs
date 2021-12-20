fn main() -> std::result::Result<(), std::io::Error> {
    let line = std::fs::read_to_string("input.txt")?;
    let notes = parse_input(&line);
    let answer1 = compute_part1(&notes);
    let answer2 = compute_part2(&notes);
    println!("part 1 answer is {}, part 2 answer is {}", answer1, answer2);
    Ok(())
}

struct NoteEntry {
    signal_patterns: Vec<String>,
    output_value: Vec<String>,
}

fn parse_note_entry(input: &str) -> NoteEntry {
    let (signal_patterns, output_value) = input.trim().split_once("|").unwrap();
    NoteEntry {
        signal_patterns: signal_patterns
            .split_ascii_whitespace()
            .map(|s| s.to_string())
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
    assert_eq!(note.signal_patterns[0], "acedgfb");
    assert_eq!(note.signal_patterns[9], "ab");
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

fn compute_part2(notes: &[NoteEntry]) -> u32 {
    0
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
fn part_2_given_example() {}
