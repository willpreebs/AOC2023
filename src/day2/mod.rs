pub fn cube_conundrum(lines: &Vec<String>) -> i32 {
    let mut sum = 0;

    for line in lines {
        // dbg!()
        sum += return_id_if_game_is_possible(line);
    }

    return sum;
}

pub fn cube_conundrum_part2(lines: &Vec<String>) -> i32 {
    let mut sum = 0;

    for line in lines {
        // dbg!()
        sum += get_power_of_fewest_number_of_cubes(line);
    }

    return sum;
}

fn get_power_of_fewest_number_of_cubes(game: &String) -> i32 {
    let dice_pulls = get_dice_pulls(&game);

    // [[(3, "blue"), (4, "red")], [(1, "red"), (2, "green")]...]
    let tuples: Vec<Vec<(i32, &str)>> = dice_pulls
        .iter()
        .map(|dice_pull| get_quanity_and_color_vector(dice_pull))
        .collect();

    let dice_pull_mininums: Vec<(i32, i32, i32)> =
        tuples.iter().map(|v| get_min_number_of_cubes(v)).collect();

    let max_tuple = find_max_of_all_elements_in_tuple(&dice_pull_mininums);

    return calculate_power(max_tuple);

    // return calculatePower(tuples.iter()
    // .fold((0, 0, 0), |acc: (i32, i32, i32), dicePull| addTuples(acc, getMinNumberOfCubes(dicePull))));
}

fn find_max_of_all_elements_in_tuple(tuples: &Vec<(i32, i32, i32)>) -> (i32, i32, i32) {
    let mut max_tuple = (0, 0, 0);

    for t in tuples {
        if t.0 > max_tuple.0 {
            max_tuple.0 = t.0;
        }
        if t.1 > max_tuple.1 {
            max_tuple.1 = t.1;
        }
        if t.2 > max_tuple.2 {
            max_tuple.2 = t.2;
        }
    }

    return max_tuple;
}

fn get_min_number_of_cubes(dice_pull: &Vec<(i32, &str)>) -> (i32, i32, i32) {
    let (mut red_count, mut blue_count, mut green_count) = (0, 0, 0);

    dice_pull.iter().for_each(|t| match t.1 {
        "red" => red_count += t.0,
        "blue" => blue_count += t.0,
        "green" => green_count += t.0,
        default => panic!("Unexpected color: {default}"),
    });

    return (red_count, blue_count, green_count);
}

fn calculate_power(cubes: (i32, i32, i32)) -> i32 {
    return cubes.0 * cubes.1 * cubes.2;
}

/**
 *
 */
fn return_id_if_game_is_possible(game: &String) -> i32 {
    let id = get_game_id(&game);

    let dice_pulls = get_dice_pulls(&game);

    let tuples: Vec<Vec<(i32, &str)>> = dice_pulls
        .iter()
        .map(|dice_pull| get_quanity_and_color_vector(dice_pull))
        .collect();

    if tuples.iter().all(|t| validate_dice_pull(t)) {
        return id;
    } else {
        return 0;
    }
}
// return true if dice pull is valid
fn validate_dice_pull(dice_pull: &Vec<(i32, &str)>) -> bool {
    let red_cubes = 12;
    let green_cubes = 13;
    let blue_cubes = 14;

    for grouping in dice_pull {
        // let red_count = 0;
        // let green_count = 0;
        // let blue_count = 0;
        match grouping.1 {
            "red" => {
                if grouping.0 > red_cubes {
                    return false;
                }
            }
            "blue" => {
                if grouping.0 > blue_cubes {
                    return false;
                }
            }
            "green" => {
                if grouping.0 > green_cubes {
                    return false;
                }
            }
            default => {
                panic!("Unexpected color {default}");
            }
        }
    }

    return true;
}

fn get_dice_pulls(game: &String) -> Vec<&str> {
    let only_games: Vec<&str> = game.split(":").skip(1).collect();
    let only_games: &str = only_games[0];

    let dice_pulls: Vec<&str> = only_games
        .split(";")
        .map(|d| d.strip_prefix(" ").unwrap())
        .collect();

    return dice_pulls.clone();
}

fn get_game_id(game: &String) -> i32 {
    let split_game: Vec<&str> = game.split(":").collect();
    let game_id_split: Vec<&str> = split_game[0].split(" ").collect();

    return game_id_split[1].parse().unwrap();
}

fn get_quanity_and_color_vector(dice_pull: &str) -> Vec<(i32, &str)> {
    let grouping: Vec<&str> = dice_pull
        .split(",")
        .map(|s| s.strip_prefix(" ").unwrap_or_else(|| s))
        .collect();

    return grouping.iter().map(|s| get_quantity_color_tuple(s)).collect();
}

// "3 blue" -> (3, "blue")
fn get_quantity_color_tuple(grouping: &str) -> (i32, &str) {
    let collection: Vec<&str> = grouping.split(" ").collect();

    return (collection[0].parse().unwrap(), collection[1]);
}

#[test]
fn test_get_game_id() {
    let input = "Game 11: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green".to_owned();

    let expected = 11;

    assert_eq!(expected, get_game_id(&input));
}

#[test]
fn test_get_dice_pulls() {
    let input = "Game 11: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green".to_owned();

    let expected = vec!["3 blue, 4 red", "1 red, 2 green, 6 blue", "2 green"];

    assert_eq!(expected, get_dice_pulls(&input));
}

#[test]
fn test_get_quantity_and_color_vector() {
    let input = "3 blue, 4 red";

    let expected = vec![(3, "blue"), (4, "red")];

    assert_eq!(expected, get_quanity_and_color_vector(input));
}

#[test]
fn test_get_quantity_color_tuple() {
    let input = "3 blue";

    let expected = (3, "blue");

    assert_eq!(expected, get_quantity_color_tuple(input));
}
