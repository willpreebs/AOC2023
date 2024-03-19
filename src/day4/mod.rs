

pub fn get_result(lines: &Vec<String>) -> i32 {

    let mut sum = 0;

    for line in lines {
        let (winning_nums, game_nums) = get_both_number_groups(line);
        sum += get_card_points(winning_nums, game_nums);
    }

    return sum;
}

pub fn get_result_part_2(lines: &Vec<String>) -> i32 {

    let mut card_instances: Vec<i32> = vec![1; lines.len()];

    for line in lines {
        let (winning_numbers, game_numbers, card_num) = get_both_number_groups_and_card_num(line);
        let card_num = card_num as usize;
        let card_index = card_num - 1;
        let num_cards_won = get_number_matches(winning_numbers, game_numbers);
        let multiplyer = card_instances[card_index];
        for i in 1..=num_cards_won {
            let i = i as usize;
            card_instances[card_index+i] += multiplyer;
        }
    }
    // dbg!(&card_instances);
    return card_instances.iter().sum();
}

fn get_both_number_groups_and_card_num(line: &String) -> (Vec<i32>, Vec<i32>, i32) {
    let nums: Vec<&str> = line.split(":").collect();
    let card_num: Vec<&str> = nums[0].split(" ").filter(|s|!s.is_empty()).collect();
    let card_num: i32 = card_num[1].parse().unwrap();
    let nums = nums[1];
    let split_nums: Vec<&str> = nums.split("|").collect();
    let winning_nums: Vec<i32> = split_into_nums(split_nums[0]);
    let game_nums: Vec<i32> = split_into_nums(split_nums[1]);
    return (winning_nums, game_nums, card_num);
}

fn get_both_number_groups(line: &String) -> (Vec<i32>, Vec<i32>) {
    let nums: Vec<&str> = line.split(":").skip(1).collect();
    let nums = nums[0];
    let split_nums: Vec<&str> = nums.split("|").collect();
    let winning_nums: Vec<i32> = split_into_nums(split_nums[0]);
    let game_nums: Vec<i32> = split_into_nums(split_nums[1]);
    return (winning_nums, game_nums);
}

fn split_into_nums(possible_nums: &str) -> Vec<i32> {
    return possible_nums.split(" ").filter_map(|s|s.trim().parse::<i32>().ok()).collect();
}

fn get_number_matches(winning_numbers: Vec<i32>, game_numbers: Vec<i32>) -> i32 {
    return game_numbers.iter().filter(|n| winning_numbers.contains(n)).count() as i32;
}

fn get_card_points(winning_numbers: Vec<i32>, game_numbers: Vec<i32>) -> i32 {

    let num_intersections = game_numbers.iter().filter(|n| winning_numbers.contains(n)).count();
    const BASE: i32 = 2;
    if num_intersections == 0 {
        return 0;
    } else {
        return BASE.pow(num_intersections as u32 - 1);
    }
}

#[test]
fn test_get_card_points() {
    
}

#[test]
fn test_get_number_groups() {
    let line = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53".to_string();

    dbg!(get_both_number_groups(&line));
}