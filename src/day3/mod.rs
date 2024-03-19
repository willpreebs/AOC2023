pub fn get_result(lines: &Vec<String>) -> i32 {

    let mut sum = 0;

    for (line_index, line) in lines.iter().enumerate() {

        sum += get_line_total(lines, line, line_index);
        
    }

    return sum;

}

pub fn get_result_part_2(lines: &Vec<String>) -> i32 {

    let mut sum = 0;

    for (line_index, line) in lines.iter().enumerate() {
        sum += get_gear_ratios_on_line(&lines, &line, line_index);
    }

    return sum;
}

fn get_gear_ratios_on_line(lines: &Vec<String>, line: &String, line_index: usize) -> i32 {

    let mut line_sum = 0;
    let optional_gear_locations: Vec<usize> = line.match_indices("*").map(|(i, _)| i).collect();

    for index in optional_gear_locations {
        let mut parts: Vec<i32> = Vec::new();
        
        let coords = get_surrounding_coords(line_index as i32, index as i32, 1);

        for coord in coords {
            if get_symbol_at_location(lines, coord.1 as usize, coord.0 as usize).is_digit(10) {
                let num = get_full_number(lines, coord);
                if !parts.contains(&num) {
                    parts.push(num);
                }
            }
        }

        if parts.len() == 2 {
            line_sum += parts[0] * parts[1];
        }
    }

    return line_sum;
}

fn get_full_number(lines: &Vec<String>, coordinate: (i32, i32)) -> i32 {
    // let start_num = get_symbol_at_coords(lines, coordinate).to_digit(10).unwrap();
    
    let mut num_string = String::from(get_symbol_at_coords(lines, coordinate).to_digit(10).unwrap().to_string());

    let mut left_coord = (coordinate.0 - 1, coordinate.1);
    // left
    while is_digit_at_coords(lines, left_coord) {
        let digit = get_symbol_at_coords(lines, left_coord);
        num_string.insert(0, digit);
        left_coord.0 -= 1;
    }
    //right
    let mut right_coord = (coordinate.0 + 1, coordinate.1);
    while is_digit_at_coords(lines, right_coord) {
        let digit = get_symbol_at_coords(lines, right_coord);
        num_string.push(digit);
        right_coord.0 += 1;
    }

    return num_string.parse().unwrap();
}

fn get_line_total(lines: &Vec<String>, line: &String, line_index: usize) -> i32 {
    let nums = get_vec_of_nums(&line);
    let mut sum = 0;
    let mut cur_digit = 0;

    for num in nums {
        let len = get_num_length(&num);
        let index = find_nth_number_index(line, &cur_digit);

        let coords = get_surrounding_coords(line_index as i32, index, len);
        if has_symbol_neighbor(&coords, &lines) {
            sum += num;
        }

        cur_digit += len;
    }

    return sum;
}

fn has_symbol_neighbor(coords: &Vec<(i32, i32)>, lines: &Vec<String>) -> bool {
    let symbols = vec!["*", "=", "/", "&", "@", "#", "-", "+", "$", "%"];

    for coord in coords {
        if coord.0 < 0 || coord.1 < 0 {
            continue;
        }

        let symbol = get_symbol_at_location(lines, coord.1 as usize, coord.0 as usize).to_string();

        if symbols.contains(&symbol.as_str()) {
            return true;
        }
    }
    return false;
}

fn get_surrounding_coords(line_index: i32, starting_row_index: i32, length: i32) -> Vec<(i32, i32)> {

    let mut coords_vec: Vec<(i32, i32)> = Vec::new();
    // left col
    
    coords_vec.push((starting_row_index-1, line_index-1));
    coords_vec.push((starting_row_index-1, line_index));
    coords_vec.push((starting_row_index-1, line_index+1));

    // right col

    coords_vec.push((starting_row_index+length, line_index-1));
    coords_vec.push((starting_row_index+length, line_index));
    coords_vec.push((starting_row_index+length, line_index+1));

    for i in 0..length {
        // top row
        coords_vec.push((starting_row_index+i, line_index-1));
        // bottom row
        coords_vec.push((starting_row_index+i, line_index+1));
    }
    
    return coords_vec;
}

fn find_nth_number_index(row: &String, n: &i32) -> i32 {

    return row.match_indices(|c: char|c.is_digit(10)).nth(n.to_owned() as usize).unwrap().0 as i32;
}

fn get_num_length(num: &i32) -> i32 {
    return num.to_string().len() as i32;
}

fn is_digit_at_coords(lines: &Vec<String>, coordinate: (i32, i32)) -> bool {
    return get_symbol_at_coords(lines, coordinate).is_digit(10);
}

fn get_symbol_at_coords(lines: &Vec<String>, coordinate: (i32, i32)) -> char {
    return get_symbol_at_location(lines, coordinate.1 as usize, coordinate.0 as usize);
}

fn get_symbol_at_location(lines: &Vec<String>, line_num: usize, row_num: usize) -> char {
    let default = "".to_owned();
    let line = lines.get(line_num).unwrap_or(&default).trim();
    let c: Vec<char> = line.chars().collect();
    // dbg!(&c);
    let r = c.get(row_num).unwrap_or(&' ');
    return *r;
}

fn get_vec_of_nums(line: &String) -> Vec<i32> {

    return line.split(|c: char| !c.is_digit(10))
    .filter(|s| !s.is_empty())
    .map(|s| s.parse::<i32>().unwrap())
    .collect();
}

#[test]
fn test_split_on_dot() {
    dbg!(get_vec_of_nums(&"..208.176..239.........*172.............795.............670.......*..9..504......*.717..641........908*....975.....*......130...............".to_owned()));
}

#[test]
fn test_get_symbol_at_coords() {

    let lines: Vec<String> = 
    "467..114..
    ...*......
    ..35..633.
    ......#...
    617*......
    .....+.58.
    ..592.....
    ......755.
    ...$.*....
    .664.598..".split("\n").map(|s| s.to_owned()).collect();

    let actual = get_symbol_at_location(&lines, 1, 3);
    let expected = '*';

    assert_eq!(expected, actual);
}

#[test]
fn test_get_line_total() {
    let lines: Vec<String> = 
    "467..114..
    ...*......
    ..35..633.
    ......#...
    617*......
    .....+.58.
    ..592.....
    ......755.
    ...$.*....
    .664.598..".split("\n").map(|s| s.to_owned()).collect();

    dbg!(&lines);

    let expected = 1263;

    let line = ".664.598..".to_owned();

    assert_eq!(expected, get_line_total(&lines, &line, 9));


}

#[test]
fn test_get_full_num() {
    let lines: Vec<String> = 
    "467..114..
    ...*......
    ..35..633.
    ......#...
    617*......
    .....+.58.
    ..592.....
    ......755.
    ...$.*....
    .664.598..".split("\n").map(|s| s.trim().to_owned()).collect();

    let expected = 467;

    let coordinate = (2, 0);

    assert_eq!(expected, get_full_number(&lines, coordinate));
}

#[test]
fn test_get_full_num2() {
    let lines: Vec<String> = 
    "467..114..
    ...*......
    ..35..633.
    ......#...
    617*......
    .....+.58.
    ..592.....
    ......755.
    ...$.*....
    .664.598..".split("\n").map(|s| s.trim().to_owned()).collect();

    let expected = 467;

    let coordinate = (0, 0);

    assert_eq!(expected, get_full_number(&lines, coordinate));
}

#[test]
fn test_get_result() {
    let lines: Vec<String> = 
    "467..114..
    ...*......
    ..35..633.
    ......#...
    617*......
    .....+.58.
    ..592.....
    ......755.
    ...$.*....
    .664.598..".split("\n").map(|s| s.trim().to_owned()).collect();

    let expected = 4361;

    assert_eq!(expected, get_result(&lines));
}