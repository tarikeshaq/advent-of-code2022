mod days;
mod utils;
fn main() {
    let mut args = std::env::args();
    // name of the program
    args.next();
    let day = args.next().unwrap();

    let file_path = format!("input/day{}.txt", day);
    let input_txt = utils::read_to_string(file_path);
    println!(
        "Q1 RESULT IS: {}",
        days::get_solver(day.parse::<u32>().unwrap()).q1(&input_txt)
    );
    println!(
        "Q2 RESULT IS: {}",
        days::get_solver(day.parse::<u32>().unwrap()).q2(&input_txt)
    );
}
