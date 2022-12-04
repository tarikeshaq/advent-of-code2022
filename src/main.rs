mod days;
mod utils;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    let mut args = std::env::args();
    // name of the program
    args.next();
    let day = args.next().unwrap().parse::<u32>().unwrap();

    let mut input_txt = utils::read_to_string(day);
    if input_txt.is_empty() {
        input_txt = utils::read_from_server(day).await;
    }
    println!("Q1 RESULT IS: {}", days::get_solver(day).q1(&input_txt));
    println!("Q2 RESULT IS: {}", days::get_solver(day).q2(&input_txt));
}
