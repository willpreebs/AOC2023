mod trebuchet1;
mod trebuchet2;

pub fn trebuchet(lines: &Vec<String>) {

    let result1: i32 = trebuchet1::get_result(&lines);
    let result2: i32 = trebuchet2::get_result(&lines);

    println!("Result part 1: {result1}");
    println!("Result part 2: {result2}");

}