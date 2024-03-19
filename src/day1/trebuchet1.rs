
pub fn get_result(lines: &Vec<String>) -> i32 {

    let mut sum: u32 = 0;

    for line in lines {

        let chars: Vec<char> = line.chars().filter(|c| c.is_numeric()).collect();

        if chars.len() == 1 {
            sum += get_char_as_num(&chars[0]) * 11;
        }
        else {
            sum += get_char_as_num(&chars[0]) * 10 + get_char_as_num(&chars[chars.len()-1]);
        }

    }

    return sum as i32;
}

fn get_char_as_num(c: &char) -> u32 {
    return c.to_digit(10).unwrap();
}