use std::env;

mod file_reader;
mod day1;
mod day2;
mod day3;
mod day4;
mod day5;

fn main() {

    let args: Vec<String> = env::args().collect();

    let day: i32 = args[1].parse().unwrap();

    let filename = format!("./inputs/{}.txt", day);
    let input_lines = file_reader::read_file_to_strings(filename.as_str());

    match day {
        1 => day1::trebuchet(&input_lines),
        2 => {
            println!("{}", day2::cube_conundrum(&input_lines));
            println!("{}", day2::cube_conundrum_part2(&input_lines));
        },
        3 => {
            println!("{}", day3::get_result(&input_lines));
            println!("{}", day3::get_result_part_2(&input_lines));
        }
        4 => {
            println!("{}", day4::get_result(&input_lines));
            println!("{}", day4::get_result_part_2(&input_lines));
        },
        _ => panic!("not done yet"),
    }
}
