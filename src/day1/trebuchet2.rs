use std::{
    cmp::{max, min},
    str::MatchIndices,
};

pub fn get_result(lines: &Vec<String>) -> i32 {
    let mut sum: i32 = 0;

    for line in lines {
        let (first, last) = find_first_and_last_numbers(line);
        sum += first * 10 + last;
    }

    return sum as i32;
}

fn find_first_and_last_numbers(line: &str) -> (i32, i32) {
    let numbers = vec![
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];

    let mut min = line.len();
    let mut min_value = 0;

    let mut max: usize = 0;
    let mut max_value = 0;

    for n in 1..10 {
        let str_rep = numbers[n - 1];
        let num_rep = n.to_string();
        let num_rep = num_rep.as_str();
        let (temp_min, temp_max) = get_min_and_max_index_pair(line, str_rep, &num_rep);

        if temp_min < min {
            min = temp_min;
            min_value = n;
        }
        if temp_max > max {
            max = temp_max;
            max_value = n;
        }
    }

    if max == 0 {
        max_value = min_value;
    } else if min == line.len() {
        min_value = max_value;
    }

    return (min_value as i32, max_value as i32);
}

fn get_min_and_max_index_pair(line: &str, str_rep: &str, num_rep: &str) -> (usize, usize) {
    let word_result = line.match_indices(str_rep);
    let num_result = line.match_indices(num_rep);

    let optional_first_str = get_first_index(word_result.clone()).unwrap_or(line.len());
    let optional_last_str = get_last_index(word_result.clone()).unwrap_or(0);

    let optional_first_num = get_first_index(num_result.clone()).unwrap_or(line.len());
    let optional_last_num = get_last_index(num_result.clone()).unwrap_or(0);

    return (
        min(optional_first_str, optional_first_num),
        max(optional_last_str, optional_last_num),
    );
}

fn get_first_index(result: MatchIndices<'_, &str>) -> Option<usize> {
    return match result.min_by(|(x, _), (y, _)| x.cmp(y)) {
        None => None,
        Some((n, _)) => return Option::Some(n),
    };
}

fn get_last_index(result: MatchIndices<'_, &str>) -> Option<usize> {
    return match result.max_by(|(x, _), (y, _)| x.cmp(y)) {
        None => None,
        Some((n, _)) => return Option::Some(n),
    };
}

#[test]
fn test_input() {
    let line = "ninesevenfivenine26c";

    assert_eq!((9, 6), find_first_and_last_numbers(line));
}
