mod day_1;
mod day_10;
mod day_11;
mod day_12;
mod day_13;
mod day_14;
mod day_15;
mod day_16;
mod day_17;
mod day_18;
mod day_19;
mod day_2;
mod day_20;
mod day_21;
mod day_22;
mod day_23;
mod day_24;
mod day_25;
mod day_3;
mod day_4;
mod day_5;
mod day_6;
mod day_7;
mod day_8;
mod day_9;

/// A trait representing a solver for each day
/// each day will have a struct that implements this trait
/// the struct will be returned as a trait object in [`get_solver`]
pub trait DaySolver {
    /// A solver for the first question of the day
    ///
    /// # Arguments:
    /// - `input_txt`: The input file parsed into a UTF-8 string
    ///
    /// # Returns:
    /// The answer to the question
    fn q1(&self, input_txt: &str) -> String;

    /// A solver for the second question of the day
    ///
    /// # Arguments:
    /// - `input_txt`: The input file parsed into a UTF-8 string
    ///
    /// # Returns:
    /// The answer to the question
    fn q2(&self, input_txt: &str) -> String;
}

/// Retrieves the solver for the day, returned as a trait object
///
/// # Arguments
/// - `day`: The day to retrieve the solver for
///
/// # Returns
/// Returns a static reference to a [`DaySolver`] trait object
pub fn get_solver(day: u32) -> &'static dyn DaySolver {
    match day {
        1 => &day_1::Solver,
        2 => &day_2::Solver,
        3 => &day_3::Solver,
        4 => &day_4::Solver,
        5 => &day_5::Solver,
        6 => &day_6::Solver,
        7 => &day_7::Solver,
        8 => &day_8::Solver,
        9 => &day_9::Solver,
        10 => &day_10::Solver,
        11 => &day_11::Solver,
        12 => &day_12::Solver,
        13 => &day_13::Solver,
        14 => &day_14::Solver,
        15 => &day_15::Solver,
        16 => &day_16::Solver,
        17 => &day_17::Solver,
        18 => &day_18::Solver,
        19 => &day_19::Solver,
        20 => &day_20::Solver,
        21 => &day_21::Solver,
        22 => &day_22::Solver,
        23 => &day_23::Solver,
        24 => &day_24::Solver,
        25 => &day_25::Solver,
        _ => panic!("Invalid day"),
    }
}
