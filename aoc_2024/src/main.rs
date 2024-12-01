#[macro_use]
extern crate log;

fn main() {
    println!("Hello, world!");
}

fn logging() {
    env_logger::init();
}

fn day_1(left_list: &mut [i32], right_list: &mut [i32]) {
    logging();
    info!("Running Day 1");
    info!("Left List  '{:?}'", left_list);
    info!("Right List '{:?}'", right_list);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day_1() {
        day_1(&mut [1, 3, 2, 5, 6], &mut [1, 3, 2, 5, 6]);
    }
}
